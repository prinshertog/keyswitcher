use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn write(cred_name: &str, key_location: &str, folder: PathBuf) {
    if folder.is_dir() {
        println!("INFO: \"credconfigs\" folder already exists");
    } else {
        create_dir_all(&folder).expect("Could not create folder");
    };

    let file = folder.join(cred_name);
    let path = Path::new(&file);
    if !path.exists() {
        let mut config_file = File::create(file).expect(&format!("Error creating config file for {}", cred_name));
        let content = format!("\
        key_location: {}", key_location);
        config_file.write_all(content.as_bytes()).expect(format!("Error writing to config file for {}", cred_name).as_str());
        println!("Config file written to {}!", cred_name);
    } else { println!("ERROR: \"configfile\" already exists") }
}

pub fn read(cred_name: &str, folder: PathBuf) -> String {
    let mut file_location = folder.clone();
    file_location.push(cred_name);
    let config_file = fs::read_to_string(file_location).map_err(|_|
        format!("Could not get config file with name {}", cred_name)).expect("File read error.");
    let borrowed_config_file = config_file.split_once(": ")
        .map(|(_, value)| value.trim().to_string())
        .expect("Invalid config format");
    borrowed_config_file
}

pub fn delete(cred_name: &str, folder: PathBuf) {
    let mut file_location = folder.clone();
    file_location.push(cred_name);
    if !file_location.exists() {
        println!("File does not exist")
    } else {
        fs::remove_file(file_location).expect("Config file does not exist");
        println!("Config was deleted successfully")
    }
}

pub fn list(folder: PathBuf) {
    let path = Path::new(&folder);
    println!("All configs:");
    if let Ok(folders) = fs::read_dir(path) {
        for folder in folders {
            if let Ok(folder) = folder {
                if let Some(filename) = folder.path().file_name() {
                    println!("- {}", filename.to_string_lossy());
                }
            }
        }
    } else {
        println!("Could not read directory.")
    }
}