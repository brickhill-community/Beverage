use std::process::Command;
use std::fs;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let path = format!("{}/.brickhill/",home);
    let client = format!("{}/.brickhill/Player.exe", home);
    let launcher = format!("{}/.brickhill/launcher", home);

    fs::create_dir_all(path)?;
    let response = get("https://brkcdn.com/client/Player.exe")?.text()?;
    fs::write(client, response)?;

    fs::write(format!("{}/.local/share/applications/brick-hill.desktop", home),
    format!("[Desktop Entry]
    Name=Brick Hill
    NoDisplay=true
    Exec={} %u
    Type=Application
    Terminal=false
    MimeType=x-scheme-handler/brickhill.legacy;", &launcher))?;

    Command::new("xdg-mime")
        .arg("default")
        .arg("brick-hill.desktop")
        .arg("x-scheme-handler/brickhill.legacy")
        .spawn()?;

    

    println!("Finished installation!");
    Ok(())
}
