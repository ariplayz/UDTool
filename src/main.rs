use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let version = "1.0.0";

    println!("UDTool v{version} by Ari Cummings");
    println!();

    if args.len() < 2 {
        println!("Invalid operator");
        println!("Usage:");
        println!("  upload <file_path> <target_name>   - Upload a file");
        println!("  download <file_name> <target_name> - Download a file");
        println!("  search <query>                      - Search for files");
        return Ok(());
    }

    let operator = &args[1];
    let client = reqwest::blocking::Client::new();

    if operator == "upload" || operator == "Upload" {
        if args.len() < 4 {
            println!("Usage: upload <file_path> <target_name>");
            return Ok(());
        }
        let file_name = &args[2];
        let target_name = &args[3];
        let file_contents = fs::read(file_name)?;

        println!("Uploading {file_name}...");
        let _res = client.post(&format!("https://UDTool.delphigamerz.xyz/{target_name}"))
            .body(file_contents)
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        println!("File successfully uploaded.");
        println!("File URL: https://UDTool.delphigamerz.xyz/{target_name}");
    }
    else if operator == "download" || operator == "Download" {
        if args.len() < 4 {
            println!("Usage: download <file_name> <target_name>");
            return Ok(());
        }
        let file_name = &args[2];
        let target_name = &args[3];

        println!("Downloading {file_name}...");
        let _res = client.get(&format!("https://UDTool.delphigamerz.xyz/{file_name}"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        println!("Downloaded {file_name}...");

        let content = _res.text().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(target_name, content)?;
    }
    else if operator == "search" || operator == "Search" {
        if args.len() < 3 {
            println!("Usage: search <query>");
            return Ok(());
        }
        let query = &args[2];

        println!("Searching for {query}...");
        let res = client.get(&format!("https://UDTool.delphigamerz.xyz/search/{query}"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        if res.status().is_success() {
            let files: Vec<String> = res.json()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            if files.is_empty() {
                println!("No files found matching '{query}'.");
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
        println!("  download <file_name> <target_name> - Download a file");
        println!("  search <query>                      - Search for files");
    }


    Ok(())
}
