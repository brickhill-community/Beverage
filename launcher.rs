fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("wine")
                         .env("WINEPROFILE", format!("{}/.brickhill/Player/", std::env::var("HOME")?))
                         .args([ format!("{}/.brickhill/Player.exe", std::env::var("HOME"))?,
                                 std::env::args().collect::<Vec<String>>()[1].replace("brickhill.legacy://client/", "")
                              ])
                         .spawn()?;
    Ok(())
}