use std::process::Command;
use std::fs;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "~/.brickhill";
    let client = "~/.brickhill/Player.exe";
    let playerprefix = "~/.brickhill/Player";
    let workshopprefix = "~/.brickhill/Workshop";

    fs::create_dir_all(path)?;
    let response = get("https://brkcdn.com/client/Player.exe")?.text()?;
    fs::write(client, response)?;

    fs::write("~/.local/share/applications/brick-hill.desktop",
    "[Desktop Entry]
    Name=Brick Hill
    NoDisplay=true
    Exec=$BRICK_HILL_PATH/launcher.sh %u
    Type=Application
    Terminal=false
    MimeType=x-scheme-handler/brickhill.legacy;")?;

    Command::new("xdg-mime")
        .arg("default")
        .arg("brick-hill.desktop")
        .arg("x-scheme-handler/brickhill.legacy")
        .spawn()?;
    Ok(())
}
