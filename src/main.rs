use std::{io::Write, process::Command};
use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

mod templates;
use templates::{get_selections, get_templates};

// https://bevyengine.org/learn/book/getting-started/setup/#compile-with-performance-optimizations
static CARGO_OPTIMIZATION : &str = r#"# Enable a small amount of optimization in debug mode
[profile.dev]
opt-level = 1

# Enable high optimizations for dependencies (incl. Bevy), but not for our code:
[profile.dev.package."*"]
opt-level = 3"#;

fn main() {

    let grey = Style::new().color256(8); // https://www.ditig.com/publications/256-colors-cheat-sheet

    println!("\n{}\n", grey.apply_to(format!("bevy-init version {}", env!("CARGO_PKG_VERSION"))));

    println!("Welcome to {}, rustaceans!", style("Bevy").bold());
    println!("The project folder will be created in the current directory.\n");

    let templates = get_templates();

    let selections = get_selections(&templates);

    println!("{}", grey.apply_to("* Your starting template"));

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which Bevy template?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let template = &templates[selection]; // The selected template

    println!("{}", grey.apply_to("\n* The name of your cargo project"));

    let name : String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Your project name?")
    .interact_text()
    .unwrap();

    println!("{}", grey.apply_to("\n* https://bevyengine.org/learn/book/getting-started/setup/#compile-with-performance-optimizations"));

    let apply_optimization =  Confirm::with_theme(&ColorfulTheme::default())
    .with_prompt("Apply debug compile performance optimizations?")
    .default(true)
    .interact()
    .unwrap();

    println!("{}", grey.apply_to("\n* You can still run Bevy with Dynamic Linking with 'cargo run --features bevy/dynamic_linking'"));
    println!("{}", grey.apply_to("  https://bevyengine.org/learn/book/getting-started/setup/#advanced-optimize-your-compilation-times"));
    
    let dynamic_link =  Confirm::with_theme(&ColorfulTheme::default())
    .with_prompt("Permanently enable Bevy's Dynamic Linking Feature?")
    .default(false)
    .interact()
    .unwrap();

    // TODO: Find out a way to open Visual Studio Code with the project
    // println!("{}", grey.apply_to("\n* Open your project folder in Visual Studio Code when ready"));
    // let open_code =  Confirm::with_theme(&ColorfulTheme::default())
    // .with_prompt("Open your project in VS Code when ready?")
    // .default(true)
    // .interact()
    // .unwrap();

    print!("\n"); // Make a new line to the cargo "created binary" message

    let mut cargo_new = Command::new("cargo") // Better way to do this??
        .args(["new", &name])
        .spawn()
        .expect("cargo new failed! do you have cargo installed?");

    if cargo_new.wait().unwrap().code().unwrap() != 0 { // Stop the execution if cargo_new failed (already a folder with the same name, etc)
        println!("{}", style("cargo new failed! Read the error message above. Stopping execution").red());
        std::process::exit(1);
    }

    // Create a path to the new project
    let path = std::path::Path::new(&name);
    let main_rs = path.join("src").join("main.rs");
    let cargo_toml = path.join("Cargo.toml");

    // Replace main.rs with the chosen template
    std::fs::write(&main_rs, &template.main_code).unwrap();

    // Append the configs to Cargo.toml
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(cargo_toml)
        .unwrap();

    if dynamic_link {
        file.write_all(format!("bevy = {{ version = \"{}\", features = [\"dynamic_linking\"] }}", template.bevy_version).as_bytes()).unwrap();
    } else {
        file.write_all(format!("bevy = \"{}\"", template.bevy_version).as_bytes()).unwrap();
    }

    if apply_optimization {
        file.write_all(format!("\n\n{}", CARGO_OPTIMIZATION).as_bytes()).unwrap();
    }

    println!("\nThe Bevy app was created in {}", style(&name).bold());
    println!("Thanks for using {}!", style("Bevy").bold());
    println!("{} {} {}", grey.apply_to("If you have Visual Studio Code installed, use"), style(format!("code {}", &name)).white(), grey.apply_to("to open the folder"));

    // Since it has already been executed by the terminal, it doesn't need to wait for input or anything like that
}
