use std::env;
use std::fs;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let version = "1.0";
    let base_url = "https://UDTool.delphigamerz.xyz";

    println!("UDTool v{version} by Ari Cummings");
    println!();

    if args.len() < 2 {
        println!("Invalid operator");
        println!("Usage:");
        println!("  upload <file_path> <target_name>    - Upload a file");
        println!("  download <file_name>                - Download a file");
        println!("  search <query>                      - Search for files");
        println!("  delete <file_name>                  - Delete a file");
        println!("  list                                - List all files");
        return Ok(());
    }

    let operator = &args[1];
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    if operator == "upload" || operator == "Upload" {
        if args.len() < 4 {
            println!("Usage: upload <file_path> <target_name>");
            return Ok(());
        }
        let file_name = &args[2];
        let target_name = &args[3];
        let file_contents = fs::read(file_name)?;

        println!("Uploading {file_name}...");
        let part = reqwest::blocking::multipart::Part::bytes(file_contents)
            .file_name(target_name.to_string())
            .mime_str("application/octet-stream")
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let form = reqwest::blocking::multipart::Form::new().part("file", part);
        let res = client.post(&format!("{base_url}/{target_name}"))
            .multipart(form)
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let status = res.status();
        if status.is_success() {
            println!("File successfully uploaded.");
            println!("File URL: {base_url}/{target_name}");
        } else {
            let body = res.text().unwrap_or_default();
            println!("Upload failed with status: {status}");
            if !body.is_empty() {
                println!("Server response: {body}");
            }
        }
    }
    else if operator == "download" || operator == "Download" {
        if args.len() < 3 {
            println!("Usage: download <file_name>");
            return Ok(());
        }
        let file_name = &args[2];

        println!("Downloading {file_name}...");
        let res = client.get(&format!("{base_url}/{file_name}"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let status = res.status();
        if status.is_success() {
            let content = res.bytes().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            fs::write(file_name, &content)?;
            println!("Downloaded {file_name}...");
        } else {
            let body = res.text().unwrap_or_default();
            println!("Download failed with status: {status}");
            if !body.is_empty() {
                println!("Server response: {body}");
            }
        }
    }
    else if operator == "search" || operator == "Search" {
        if args.len() < 3 {
            println!("Usage: search <query>");
            return Ok(());
        }
        let query = &args[2];

        println!("Searching for {query}...");
        let res = client.get(&format!("{base_url}/search/{query}"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let status = res.status();
        if status.is_success() {
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
            let body = res.text().unwrap_or_default();
            println!("Search failed with status: {status}");
            if !body.is_empty() {
                println!("Server response: {body}");
            }
        }
    }
    else if operator == "delete" || operator == "Delete" {
        if args.len() < 3 {
            println!("Usage: delete <file_name>");
            return Ok(());
        }
        let file_name = &args[2];

        println!("Deleting {file_name}...");
        let res = client.delete(&format!("{base_url}/{file_name}"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let status = res.status();
        if status.is_success() {
            println!("File '{file_name}' successfully deleted.");
        } else {
            let body = res.text().unwrap_or_default();
            println!("Delete failed with status: {status}");
            if !body.is_empty() {
                println!("Server response: {body}");
            }
        }

    }
    else if operator == "list" || operator == "List" {
        println!("Listing all files...");
        let res = client.get(&format!("{base_url}/list"))
            .send()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let status = res.status();
        if status.is_success() {
            let files: Vec<String> = res.json()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            if files.is_empty() {
                println!("No files found.");
            } else {
                println!("Files:");
                for file in files {
                    println!("  - {}", file);
                }
            }
        } else {
            let body = res.text().unwrap_or_default();
            println!("List failed with status: {status}");
            if !body.is_empty() {
                println!("Server response: {body}");
            }
        }
    }
    else {
        println!("Invalid operator: {operator}");
        println!("Usage:");
        println!("  upload <file_path> <target_name>    - Upload a file");
        println!("  download <file_name>                - Download a file");
        println!("  search <query>                      - Search for files");
        println!("  delete <file_name>                  - Delete a file");
        println!("  list                                - List all files");
    }


    Ok(())
}
