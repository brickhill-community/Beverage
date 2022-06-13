fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments: Vec<String> = Vec::new();
    for argument in std::env::args() { arguments.push(argument) }

    std::process::Command::new("wine")
                        .env("WINEPROFILE", format!("{}/.brickhill/Player/", std::env::var("HOME")?))
                        .arg(format!("{}/.brickhill/Player.exe", std::env::var("HOME")?))
                        .arg(arguments[1].replace("brickhill.legacy://client/", ""))
                        .spawn()?;
    Ok(())
}