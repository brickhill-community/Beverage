use std::process::Command;
use std::env::args;
use std::error::Error;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn player() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect::<Vec<String>>();
    let home = std::env::var("HOME")?;
    Command::new("wine")
            .env("WINEPROFILE", format!("{}/.brickhill/Player", home))
            .args([ format!("{}/.brickhill/Player.exe", home),
                    args[2].replace("brickhill.legacy://client/", "")
                 ])
            .spawn()?;
    Ok(())
}

fn workshop() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect::<Vec<String>>();
    let home = std::env::var("HOME")?;
    Command::new("wine")
            .env("WINEPROFILE", format!("{}/.brickhill/Workshop", home))
            .args([ format!("{}/.brickhill/Workshop.exe", home),
                    args[2].replace("brickhill.legacy://workshop/", "")
                 ])
            .spawn()?;
    Ok(())
}

fn help(errtype: &str) {
    println!("\x1b[91mError:\x1b[0m {}\n", errtype);
    println!("beverage   <SUBCOMMAND> [ARGUMENTS]");
    println!("player   | Starts the BrickHill client");
    println!("workshop | Starts the BrickHill workshop");
    std::process::exit(0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect::<Vec<String>>();
    if args.len() < 2 { help("No arguments were given") }

    match args[1].as_str() {
        "player" => player()?,
        "workshop" => workshop()?,
        _ => help("Unknown argument")
    }
    Ok(())
}