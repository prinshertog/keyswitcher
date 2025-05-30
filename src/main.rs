use std::{env, fs};
use std::fs::{File, create_dir_all};
use std::io::Write;
use dirs::home_dir;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let mut folder = home_dir().unwrap();
    folder.push(".config/gitkeyswitcher/credconfigs");
    // A vector is like an array but scales at runtime
    let args: Vec<String> = env::args().collect();

    // These are the parameters given when executing the program
    // When there are no parameters show the help output
    let help_message = "No parameter supplied\n\
                              Usage:\n\
                              - save (name) (key_locations_path)\n\
                              - load (name)\n\
                              - delete (name)\n\
                              - list";
    let super_parameter = args.get(1).expect(help_message);

    // Check if the "save" parameter is given
    if super_parameter == "save" {
        let cred_name = args.get(2).expect("Please supply a credential name");
        let key_location = args.get(3).expect("Please supply a key location");
        let mut key_file_location = key_location.clone();
        key_file_location.push_str(&cred_name);
        if !Path::new(&key_location).exists() {
            println!("Key does not exist at location: {}", key_location);
        } else {
            write_config_to_file(cred_name, key_location, folder);
        };
    } else if super_parameter == "load" {
        let cred_name = args.get(2).expect("Please supply a credential name");
        let key_file_path = read_config_file(cred_name, folder);
        if !Path::new(&key_file_path).exists() { println!("Key does not exist at location: {}", key_file_path)};
        let command = Command::new("bash")
            .arg("-c")
            .arg(&format!("ssh-add {}", key_file_path))
            .output();
        match command {
            Ok(output) if !output.stderr.is_empty() => {
                println!("Error running ssh-add:\n{}", String::from_utf8_lossy(&output.stderr));
            },
            Ok(output) => {
                println!("SSH key added successfully.");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            Err(err) => println!("Failed to execute ssh-add: {}", err),
        }
    } else if super_parameter == "delete" {
        let cred_name = args.get(2).expect("Please supply a credential name");
        delete_config_file(cred_name, folder)
    } else if super_parameter == "list" {
        list_config_files(folder);
    } else {
        println!("Invalid parameter");
    };
}

fn write_config_to_file(cred_name: &str, key_location: &str, folder: PathBuf) {
    if folder.is_dir() {
        println!("INFO: \"credconfigs\" folder already exists");
    } else {
        create_dir_all(&folder).expect("Could not create folder");
    };

    // Creation of the config file
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

fn read_config_file(cred_name: &str, folder: PathBuf) -> String {
    let mut file_location = folder.clone();
    file_location.push(cred_name);
    let config_file = fs::read_to_string(file_location).map_err(|_| format!("Could not get config file with name {}", cred_name)).expect("File read error.");
    let borrowed_config_file = config_file.split_once(": ")
        .map(|(_, value)| value.trim().to_string())
        .expect("Invalid config format");
    borrowed_config_file
}

fn delete_config_file(cred_name: &str, folder: PathBuf) {
    let mut file_location = folder.clone();
    file_location.push(cred_name);
    if !file_location.exists() {
        println!("File does not exist")
    } else {
        fs::remove_file(file_location).expect("Config file does not exist");
        println!("Config was deleted successfully")
    }
}

fn list_config_files(folder: PathBuf) {
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