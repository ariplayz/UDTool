use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let operator = &args[1];
    let file_contents = fs::read(&args[2])?;
    let file_name = &args[2];
    let version = "1.0.0";
    let client = reqwest::blocking::Client::new();

    println!("UDTool v{version} by Ari Cummings");
    println!();

    if operator == "upload" || operator == "Upload" {
        println!("Uploading {file_name}...");
        let _res = client.post("https://UDTool.delphigamerz.xyz/{file_name}")
            .body(file_contents)
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        println!("File successfully uploaded.");
        println!("File URL: https://UDTool.delphigamerz.xyz/{file_name}");
    }
    else if operator == "download" || operator == "Download" {
        println!("Downloading {file_name}...");
        let _res = client.get("https://UDTool.delphigamerz.xyz/{file_name}")
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        println!("Downloaded {file_name}...");

        let content = _res.text().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(&args[2], content)?;
    }



    Ok(())
}
