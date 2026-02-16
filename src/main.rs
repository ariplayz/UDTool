use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_contents = fs::read(&args[1])?;

    let client = reqwest::blocking::Client::new();
    let _res = client.post("https://UDTool.delphigamerz.xyz")
        .body(file_contents)
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;



    Ok(())
}
