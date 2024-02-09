use std::{io::Write, process::Command};
use clap::ArgAction;
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

fn cli() -> clap::Command {
    clap::Command::new("Bevy Init")
        .version(env!("CARGO_PKG_VERSION")) // Bump version here too
        .author("nigro.dev")
        .about("An easy way to set up a Bevy Engine project")
        .arg(
            clap::Arg::new("offline")
                .short('o')
                .long("offline")
                .help("It doesn't look for new templates online and only shows embedded ones")
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            clap::Command::new("create")
                .about("Create a new project using Bevy with useful templates")
                .subcommand(
                    clap::Command::new("minimal")
                    .short_flag('m')
                    .long_flag("minimal")
                    .about("Skip the options and templates to create a minimal project straight away")
                )
        )
}

#[tokio::main]
async fn main() {

    // https://github.com/console-rs/dialoguer/issues/294
    // Ignore SIGINT so we can handle it ourselves
    // On Linux, it ignores Ctrl+C and continues the program
    // On Windows, it simply freezes
    // I don't know what happens on macOS, please open an issue if something related happened to you
    ctrlc::set_handler( || {
        if cfg!(windows) {
            console::Term::stdout().show_cursor().unwrap();
            std::process::exit(0);
        }
    }).expect("Error setting Ctrl-C handler");

    let matches = cli().get_matches();

    let grey = Style::new().color256(8); // https://www.ditig.com/publications/256-colors-cheat-sheet

    println!("\n{}\n", grey.apply_to(format!("bevy-init version {}", env!("CARGO_PKG_VERSION"))));

    println!("Welcome to {}, rustaceans!\n", style("Bevy").bold());

    let offline_mode : &bool = matches.get_one("offline").unwrap_or(&false);

    match matches.subcommand() {
        Some(("create", query_matches)) => {
            match query_matches.subcommand() {
                Some(("minimal", _)) => {
                    // 0 should be the first and minimal example
                    // as seen in template "force_order: Some(0)"
                    let _ = setup_project(Some(0), offline_mode).await;
                }
                _ => {
                    // Default behavior if no subcommand is provided
                    let _ = setup_project(None, offline_mode).await;
                }
            }
        }
        _ => {
            // Default behavior if no subcommand is provided
            let _ = choose_mode(offline_mode).await;
        }
    }

    // Since it has already been executed by the terminal, it doesn't need to wait for input or anything like that
}

async fn choose_mode(offline_mode : &bool) -> Result<(), dialoguer::Error> {
    let items = vec!["Project Setup"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do with bevyinit?")
        .items(&items)
        .default(0)
        .interact()
        .map_err(|e| {
            let _ = console::Term::stdout().show_cursor();
            e
        })?;

    print!("\n");

    let _ = match selection {
        0 => setup_project(None, offline_mode).await,
        _ => unreachable!()
    };

    Ok(())
}

async fn setup_project(selected_theme : Option<usize>, offline_mode : &bool) -> Result<(), dialoguer::Error> {

    let grey = Style::new().color256(8); // https://www.ditig.com/publications/256-colors-cheat-sheet

    println!("The project folder will be created in the current directory.\n");

    let templates = get_templates(!offline_mode).await.expect("Error getting templates");

    let template : &templates::Template;
    let dynamic_link : bool;
    let apply_optimization : bool;
    let name : String;

    let selections = get_selections(&templates);

    if let Some(theme) = selected_theme {

        template = &templates[theme];
        dynamic_link = false;
        apply_optimization = true;

        println!("{}", grey.apply_to("* The name of your cargo project"));

        // Duplicated name input
        name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your project name?")
        .interact_text()
        .map_err(|e| {
            let _ = console::Term::stdout().show_cursor();
            e
        })?;

    } else {

        println!("{}", grey.apply_to("* Your starting template"));

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which Bevy template?")
            .default(0)
            .items(&selections[..])
            .interact()
            .map_err(|e| {
                let _ = console::Term::stdout().show_cursor();
                e
            })?;

        template = &templates[selection]; // The selected template

        println!("{}", grey.apply_to("\n* The name of your cargo project"));

        name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your project name?")
        .interact_text()
        .map_err(|e| {
            let _ = console::Term::stdout().show_cursor();
            e
        })?;

        println!("{}", grey.apply_to("\n* https://bevyengine.org/learn/book/getting-started/setup/#compile-with-performance-optimizations"));

        apply_optimization =  Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Apply debug compile performance optimizations?")
        .default(true)
        .interact()
        .map_err(|e| {
            let _ = console::Term::stdout().show_cursor();
            e
        })?;

        println!("{}", grey.apply_to("\n* You can still run Bevy with Dynamic Linking with 'cargo run --features bevy/dynamic_linking'"));
        println!("{}", grey.apply_to("  https://bevyengine.org/learn/book/getting-started/setup/#advanced-optimize-your-compilation-times"));
        
        dynamic_link =  Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Permanently enable Bevy's Dynamic Linking Feature?")
        .default(false)
        .interact()
        .map_err(|e| {
            let _ = console::Term::stdout().show_cursor();
            e
        })?;

        // TODO: Find out a way to open Visual Studio Code with the project
        // println!("{}", grey.apply_to("\n* Open your project folder in Visual Studio Code when ready"));
        // let open_code =  Confirm::with_theme(&ColorfulTheme::default())
        // .with_prompt("Open your project in VS Code when ready?")
        // .default(true)
        // .interact()
        // .unwrap();

    }

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

    for crate_ in &template.extra_crates {
        file.write_all(format!("\n{} = \"{}\"", crate_.0, crate_.1).as_bytes()).unwrap();
    }

    if apply_optimization {
        file.write_all(format!("\n\n{}", CARGO_OPTIMIZATION).as_bytes()).unwrap();
    }

    println!("\nThe Bevy app was created in {}", style(&name).bold());
    println!("Thanks for using {}!", style("Bevy").bold());
    println!("{} {} {}", grey.apply_to("If you have Visual Studio Code installed, use"), style(format!("code {}", &name)).white(), grey.apply_to("to open the folder"));

    Ok(())
}