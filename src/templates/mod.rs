use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    beauty_name : String,
    force_order : Option<usize>,
    pub bevy_version : String,
    pub main_code : String
}

#[derive(RustEmbed)]
#[folder = "templates/"] // The templates are embedded on build by rust_embed
struct Asset;

pub fn get_selections(templates : &Vec<Template>) -> Vec<String> {
    let mut selections : Vec<String> = Vec::new();

    for template in templates.iter() {
        selections.push(format!("{} ({})", template.beauty_name, template.bevy_version));
    }

    selections
}

pub fn get_templates() -> Vec<Template> {

    let mut templates : Vec<Template> = Vec::new();

    for file in Asset::iter() {

        let content = Asset::get(file.as_ref()).unwrap();
        let content_str = std::str::from_utf8(content.data.as_ref()).unwrap();
        
        let x : Template = ron::from_str(content_str).unwrap();

        if let Some(order) = x.force_order {
            templates.insert(order, x);
        } else {
            templates.push(x);
        }
    }

    templates
}