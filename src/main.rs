use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let operator = &args[1];
    let file_contents = fs::read(&args[2])?;
    let file_name = &args[2];
    let target_name = &args[3];
    let version = "1.0.0";
    let client = reqwest::blocking::Client::new();

    println!("UDTool v{version} by Ari Cummings");
    println!();

    if operator == "upload" || operator == "Upload" {
        println!("Uploading {file_name}...");
        let _res = client.post(&format!("https://UDTool.delphigamerz.xyz/{target_name}"))
            .body(file_contents)
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        println!("File successfully uploaded.");
        println!("File URL: https://UDTool.delphigamerz.xyz/{target_name}");
    }
    else if operator == "download" || operator == "Download" {
        println!("Downloading {file_name}...");
        let _res = client.get(&format!("https://UDTool.delphigamerz.xyz/{file_name}"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        println!("Downloaded {file_name}...");

        let content = _res.text().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(&args[2], content)?;
    }
    else if operator == "search" || operator == "Search" {
        println!("Searching for {file_name}...");
        let res = client.get(&format!("https://UDTool.delphigamerz.xyz/search/{file_name}"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        if res.status().is_success() {
            let files: Vec<String> = res.json()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            if files.is_empty() {
                println!("No files found matching '{file_name}'.");
            } else {
                println!("Found {} file(s):", files.len());
                for file in files {
                    println!("  - {}", file);
                }
            }
        } else {
            println!("Search failed with status: {}", res.status());
        }
    }
    else {
        println!("Invalid operator: {operator}");
        println!("Usage:");
        println!("  upload <file_path> <target_name>   - Upload a file");
        println!("  download <file_name> - Download a file");
        println!("  search <query>       - Search for files");
    }


    Ok(())
}
