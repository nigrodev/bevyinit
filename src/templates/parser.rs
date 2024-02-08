use super::Template;

pub fn parse_template(raw : String) -> Template {
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

    let mut template : Template = ron::from_str(above_code.trim()).expect("Failed to parse ron template");

    template.main_code = Some(below_code);

    template
}