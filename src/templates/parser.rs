use std::collections::HashMap;

use serde::Deserialize;
use super::Template;

#[derive(Deserialize)]
struct TemplateFile {
    beauty_name: String,
    force_order: Option<usize>,
    pub bevy_version: String,
    extra_crates : HashMap<String, String>,
}

pub fn parse_template(raw : String, from_online_data : bool) -> Template {
    // Find the position of "--!code" in the file
    let mut above_code = String::new();
    let mut below_code = String::new();
    let mut found_code_marker = false;
    
    for line in raw.lines() {
        if !found_code_marker {
            if line.contains("--!code") {
                found_code_marker = true;
            } else {
                above_code.push_str(&line);
                above_code.push('\n');
            }
        } else {
            below_code.push_str(&line);
            below_code.push('\n');
        }
    }

    // Remove all extra space
    below_code = below_code.trim().to_string();

    let template : TemplateFile = ron::from_str(above_code.trim()).expect("Failed to parse ron template");

    Template {
        beauty_name: template.beauty_name,
        force_order: template.force_order,
        bevy_version: template.bevy_version,
        main_code: below_code,
        from_online_data,
        extra_crates: template.extra_crates
    }
}