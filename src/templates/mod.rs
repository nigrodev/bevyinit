use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

use self::parser::parse_template;

mod online;
mod parser;

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    beauty_name : String,
    force_order : Option<usize>,
    pub bevy_version : String,
    pub main_code : String,
    from_online_data : bool,
}

#[derive(RustEmbed)]
#[folder = "templates/"] // The templates are embedded on build by rust_embed
struct Asset;

pub fn get_selections(templates : &Vec<Template>) -> Vec<String> {
    let mut selections : Vec<String> = Vec::new();

    for template in templates.iter() {
        selections.push(format!("{} ({}) [{}]", template.beauty_name, template.bevy_version, if template.from_online_data { "online" } else { "included" }));
    }

    selections
}

pub async fn get_templates(get_online : bool) -> Result<Vec<Template>, Box<dyn std::error::Error>> {

    let mut templates : Vec<Template> = Vec::new();

    for file in Asset::iter() {

        let content = Asset::get(file.as_ref()).expect("Failed to read file");
        let content_str = std::str::from_utf8(content.data.as_ref())?;
        
        let template = parse_template(content_str.to_string(), false);

        if let Some(order) = template.force_order {
            templates.insert(order, template);
        } else {
            templates.push(template);
        }
    }

    if get_online {
        if online::get_online(&mut templates).await.is_err() {
            println!("{} {}", console::style("[!]").red().bold(), console::style("The request for the online data went wrong. This is to be expected if you don't have internet access. Otherwise, please open an issue in the GitHub repository.").bold());
            println!("{}\n", console::style("Only templates already saved locally or in the binary will be displayed.").bold());
        }
    } else {
        println!("{}\n", console::style("Only templates already saved locally or in the binary will be displayed.").bold());
    }

    Ok(templates)
}