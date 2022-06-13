use std::process::Command;
use std::fs;
use std::io::Write;
use curl::easy::Easy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reqclient = Easy::new();
    let home = std::env::var("HOME")?;
    let path = format!("{}/.brickhill/Player/",home);
    let client = format!("{}/.brickhill/Player.exe", home);
    let launcher = format!("{}/.brickhill/launcher", home);

    fs::create_dir_all(path)?;
    let mut clientfile = fs::File::create(client)?;
    reqclient.url("https://brkcdn.com/client/Player.exe")?;
    reqclient.write_function(move |data| {
        clientfile.write_all(data).unwrap();
        Ok(data.len())
    })?;
    reqclient.perform()?;

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

    let mut launcherfile = fs::File::create(&launcher)?;
    reqclient.url("https://brkcdn.com/client/Player.exe")?;
    reqclient.write_function(move |data| {
        launcherfile.write_all(data).unwrap();
        Ok(data.len())
    })?;
    reqclient.perform()?;

    Command::new("chmod")
            .arg("+x")
            .arg(launcher)
            .spawn()?;

    println!("Finished installation!");
    Ok(())
}
