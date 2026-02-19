use std::env;
use std::fs;
use std::path::PathBuf;

fn get_api_key_file() -> PathBuf {
    if let Some(data_dir) = dirs::config_dir() {
        data_dir.join("UDTool").join("api_key.txt")
    } else {
        PathBuf::from("api_key.txt")
    }
}

fn load_api_key() -> std::io::Result<String> {
    let api_key_file = get_api_key_file();
    if api_key_file.exists() {
        let key = fs::read_to_string(&api_key_file)?;
        Ok(key.trim().to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "API key not found. Use 'genkey' command to generate a new key.",
        ))
    }
}

fn save_api_key(key: &str) -> std::io::Result<()> {
    let api_key_file = get_api_key_file();
    if let Some(parent) = api_key_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&api_key_file, key)?;
    Ok(())
}

fn check_key(args: &[String], client: &reqwest::blocking::Client, base_url: &str) -> std::io::Result<()> {
    let key = if args.len() < 3 {
        load_api_key()?
    } else {
        args[2].clone()
    };

    println!("Checking API key...");
    let res = client
        .get(&format!("{base_url}/key/check/{key}"))
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match res.status().is_success() {
        true => handle_check_key_success(res, &key)?,
        false => handle_check_key_error(res)?,
    }
    Ok(())
}

fn handle_check_key_success(res: reqwest::blocking::Response, key: &str) -> std::io::Result<()> {
    match res.json::<serde_json::Value>() {
        Ok(json) => {
            if let Some(message) = json.get("message").and_then(|m| m.as_str()) {
                println!("{}", message);
                if message == "Key is valid." {
                    save_api_key(key)?;
                    println!("API key saved successfully.");
                }
            }
        }
        Err(_) => println!("Could not parse response."),
    }
    Ok(())
}

fn handle_check_key_error(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let status = res.status();
    let body = res.text().unwrap_or_default();
    println!("Check failed with status: {status}");
    if !body.is_empty() {
        println!("Server response: {body}");
    }
    Ok(())
}

fn generate_key(client: &reqwest::blocking::Client, base_url: &str) -> std::io::Result<()> {
    println!("Generating new API key...");
    let res = client
        .post(&format!("{base_url}/key/new"))
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match res.status().is_success() {
        true => handle_generate_key_success(res)?,
        false => handle_generate_key_error(res)?,
    }
    Ok(())
}

fn handle_generate_key_success(res: reqwest::blocking::Response) -> std::io::Result<()> {
    match res.json::<serde_json::Value>() {
        Ok(json) => {
            match json.get("key").and_then(|k| k.as_str()) {
                Some(key) => {
                    println!("New API key generated: {key}");
                    save_api_key(key)?;
                    println!("API key saved successfully.");
                }
                None => println!("Could not extract key from response."),
            }
        }
        Err(_) => println!("Could not parse response."),
    }
    Ok(())
}

fn handle_generate_key_error(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let status = res.status();
    let body = res.text().unwrap_or_default();
    println!("Key generation failed with status: {status}");
    if !body.is_empty() {
        println!("Server response: {body}");
    }
    Ok(())
}

fn upload(args: &[String], client: &reqwest::blocking::Client, base_url: &str, api_key: &str) -> std::io::Result<()> {
    if args.len() < 4 {
        println!("Usage: upload <file_path> <target_name>");
        return Ok(());
    }
    let file_path = &args[2];
    let target_name = &args[3];

    // Get file metadata for progress bar
    let metadata = fs::metadata(file_path)?;
    let total_size = metadata.len();

    println!("Uploading {file_path}...");

    // Create progress bar
    let pb = indicatif::ProgressBar::new(total_size);
    pb.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Read file in chunks and update progress
    let file_contents = fs::read(file_path)?;
    pb.inc(file_contents.len() as u64);

    // Create multipart form with correct field name "file"
    let part = reqwest::blocking::multipart::Part::bytes(file_contents)
        .file_name(target_name.to_string());

    let form = reqwest::blocking::multipart::Form::new()
        .part("file", part);

    let res = client
        .post(&format!("{base_url}/{target_name}"))
        .header("API-Key", api_key)
        .multipart(form)
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    pb.finish_with_message("Upload complete");

    match res.status().is_success() {
        true => handle_upload_success(res, base_url, target_name)?,
        false => handle_upload_error(res)?,
    }
    Ok(())
}

fn handle_upload_success(_res: reqwest::blocking::Response, base_url: &str, target_name: &str) -> std::io::Result<()> {
    println!("File successfully uploaded.");
    println!("File URL: {base_url}/{target_name}");
    Ok(())
}

fn handle_upload_error(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let status = res.status();
    let body = res.text().unwrap_or_default();
    println!("Upload failed with status: {status}");
    if !body.is_empty() {
        println!("Server response: {body}");
    }
    Ok(())
}



fn download(args: &[String], client: &reqwest::blocking::Client, base_url: &str, api_key: &str) -> std::io::Result<()> {
    if args.len() < 3 {
        println!("Usage: download <file_name>");
        return Ok(());
    }
    let file_name = &args[2];

    println!("Downloading {file_name}...");
    let res = client
        .get(&format!("{base_url}/{file_name}"))
        .header("API-Key", api_key)
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match res.status().is_success() {
        true => handle_download_success(res, file_name)?,
        false => handle_download_error(res)?,
    }
    Ok(())
}

fn handle_download_success(res: reqwest::blocking::Response, file_name: &str) -> std::io::Result<()> {
    // Get content length for progress bar
    let total_size = res.content_length().unwrap_or(0);

    // Create progress bar
    let pb = indicatif::ProgressBar::new(total_size);
    pb.set_style(indicatif::ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Read content and update progress
    let content = res.bytes().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    pb.inc(content.len() as u64);
    pb.finish_with_message("Download complete");

    fs::write(file_name, &content)?;
    println!("Downloaded {file_name} successfully.");
    Ok(())
}

fn handle_download_error(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let status = res.status();
    let body = res.text().unwrap_or_default();
    println!("Download failed with status: {status}");
    if !body.is_empty() {
        println!("Server response: {body}");
    }
    Ok(())
}

fn search(args: &[String], client: &reqwest::blocking::Client, base_url: &str, api_key: &str) -> std::io::Result<()> {
    if args.len() < 3 {
        println!("Usage: search <query>");
        return Ok(());
    }
    let query = &args[2];

    println!("Searching for {query}...");
    let res = client
        .get(&format!("{base_url}/search/{query}"))
        .header("API-Key", api_key)
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match res.status().is_success() {
        true => handle_search_success(res, query)?,
        false => handle_search_error(res)?,
    }
    Ok(())
}

fn handle_search_success(res: reqwest::blocking::Response, query: &str) -> std::io::Result<()> {
    let files: Vec<String> = res.json()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match files.is_empty() {
        true => println!("No files found matching '{query}'."),
        false => {
            println!("Found {} file(s):", files.len());
            for file in files {
                println!("  - {}", file);
            }
        }
    }
    Ok(())
}

fn handle_search_error(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let status = res.status();
    let body = res.text().unwrap_or_default();
    println!("Search failed with status: {status}");
    if !body.is_empty() {
        println!("Server response: {body}");
    }
    Ok(())
}

fn delete(args: &[String], client: &reqwest::blocking::Client, base_url: &str, api_key: &str) -> std::io::Result<()> {
    if args.len() < 3 {
        println!("Usage: delete <file_name>");
        return Ok(());
    }
    let file_name = &args[2];

    println!("Deleting {file_name}...");
    let res = client
        .delete(&format!("{base_url}/{file_name}"))
        .header("API-Key", api_key)
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match res.status().is_success() {
        true => handle_delete_success(file_name)?,
        false => handle_delete_error(res)?,
    }
    Ok(())
}

fn handle_delete_success(file_name: &str) -> std::io::Result<()> {
    println!("File '{file_name}' successfully deleted.");
    Ok(())
}

fn handle_delete_error(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let status = res.status();
    let body = res.text().unwrap_or_default();
    println!("Delete failed with status: {status}");
    if !body.is_empty() {
        println!("Server response: {body}");
    }
    Ok(())
}

fn list_files(client: &reqwest::blocking::Client, base_url: &str, api_key: &str) -> std::io::Result<()> {
    println!("Listing all files...");
    let res = client
        .get(&format!("{base_url}/list"))
        .header("API-Key", api_key)
        .send()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match res.status().is_success() {
        true => handle_list_success(res)?,
        false => handle_list_error(res)?,
    }
    Ok(())
}

fn handle_list_success(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let files: Vec<String> = res.json()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    match files.is_empty() {
        true => println!("No files found."),
        false => {
            println!("Files:");
            for file in files {
                println!("  - {}", file);
            }
        }
    }
    Ok(())
}

fn handle_list_error(res: reqwest::blocking::Response) -> std::io::Result<()> {
    let status = res.status();
    let body = res.text().unwrap_or_default();
    println!("List failed with status: {status}");
    if !body.is_empty() {
        println!("Server response: {body}");
    }
    Ok(())
}


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let version = env!("CARGO_PKG_VERSION");
    let base_url = "https://UDTool.delphigamerz.xyz";

    println!("UDTool v{version} by Ari Cummings");
    println!();

    if args.len() < 2 {
        println!("Invalid operator");
        print_usage();
        return Ok(());
    }

    let operator = &args[1];
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(25200))
        .build()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let cmd = operator.to_lowercase();
    match cmd.as_str() {
        "genkey" => generate_key(&client, base_url)?,
        "checkkey" => check_key(&args, &client, base_url)?,
        "upload" => {
            let api_key = load_api_key()?;
            upload(&args, &client, base_url, &api_key)?
        }
        "download" => {
            let api_key = load_api_key()?;
            download(&args, &client, base_url, &api_key)?
        }
        "search" => {
            let api_key = load_api_key()?;
            search(&args, &client, base_url, &api_key)?
        }
        "delete" => {
            let api_key = load_api_key()?;
            delete(&args, &client, base_url, &api_key)?
        }
        "list" => {
            let api_key = load_api_key()?;
            list_files(&client, base_url, &api_key)?
        }
        _ => {
            println!("Invalid operator: {operator}");
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  genkey                              - Generate a new API key");
    println!("  checkkey <api_key>                  - Check if an API key is valid");
    println!("  upload <file_path> <target_name>    - Upload a file");
    println!("  download <file_name>                - Download a file");
    println!("  search <query>                      - Search for files");
    println!("  delete <file_name>                  - Delete a file");
    println!("  list                                - List all files");
}
