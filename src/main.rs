use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let operator = &args[1];
    let file_contents = fs::read(&args[2])?;
    let file_name = &args[2];

    let client = reqwest::blocking::Client::new();

    if operator == "upload" || operator == "Upload" {
        let _res = client.post("https://UDTool.delphigamerz.xyz/{file_name}")
            .body(file_contents)
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    }
    else if operator == "download" || operator == "Download" {
        let res = client.get("https://UDTool.delphigamerz.xyz/{file_name}")
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let content = res.text().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(&args[2], content)?;
    }



    Ok(())
}
