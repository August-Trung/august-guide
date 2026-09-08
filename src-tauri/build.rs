use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("credentials.rs");

    let mut client_id = String::new();
    let mut client_secret = String::new();

    let env_paths = [Path::new(".env"), Path::new("../.env")];
    let mut env_found = false;

    for path in &env_paths {
        if path.exists() {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for line in reader.lines().map_while(Result::ok) {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some((key, value)) = trimmed.split_once('=') {
                        let key = key.trim();
                        let value = value.trim().trim_matches('"').trim_matches('\'');
                        if key == "GOOGLE_CLIENT_ID" {
                            client_id = value.to_string();
                        } else if key == "GOOGLE_CLIENT_SECRET" {
                            client_secret = value.to_string();
                        }
                    }
                }
            }
            println!("cargo:rerun-if-changed={}", path.display());
            env_found = true;
            break;
        }
    }

    // If no .env file was found, tell cargo to rerun if the env files are created
    if !env_found {
        for path in &env_paths {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Write the credentials.rs file
    let credentials_content = format!(
        "pub const GOOGLE_CLIENT_ID: &str = {:?};\npub const GOOGLE_CLIENT_SECRET: &str = {:?};\n",
        client_id, client_secret
    );
    fs::write(&dest_path, credentials_content).unwrap();

    tauri_build::build();
}
