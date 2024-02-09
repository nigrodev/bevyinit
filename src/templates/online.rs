use serde::Deserialize;

use super::{parser::parse_template, Template};

// Using github raw url. This could cause some problems when changing name, file order, etc.
// Always get from latest version (main)
static RAW_URL_BASE: &str = "https://raw.githubusercontent.com/nigrodev/bevyinit_data/main";

#[derive(Deserialize, Debug)]
struct BevyInitData {
    bevyinit: Config,
    data: Data,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Config {
   version : String,
   bin_version : String,
   template_path : String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Data {
   templates : Vec<String>,
   crates : Vec<String>,
   repos : Vec<String>,
}

pub async fn get_online(templates : &mut Vec<Template>) -> Result<(), Box<dyn std::error::Error>> {
   
   let resp = reqwest::get(format!("{RAW_URL_BASE}/data.toml"))
       .await?
       .text()
       .await?;

   let config: BevyInitData = toml::from_str(&resp)?;
   
   let templates_url = format!("{RAW_URL_BASE}/{}", config.bevyinit.template_path);

   for template in config.data.templates {
       let template_url = format!("{templates_url}/{template}.ron"); // File type is ron

       let resp = reqwest::get(template_url)
           .await?
           .text()
           .await?;

       let template = parse_template(resp, true);
       templates.push(template);
   }

   Ok(())
}