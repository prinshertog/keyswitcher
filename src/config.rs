use std::path::{Path, PathBuf};
use std::process::Command;
use crate::{file};

pub fn save(args: Vec<String>, folder: PathBuf) {
    let cred_name = args.get(2).expect("Please supply a credential name");
    let key_location = args.get(3).expect("Please supply a key location");
    
    let mut key_file_location = key_location.clone();
    key_file_location.push_str(&cred_name);

    if !Path::new(&key_location).exists() {
        println!("Key does not exist at location: {}", key_location);
    } else {
        file::write(cred_name, key_location, folder);
    };
}

pub fn load(args: Vec<String>, folder: PathBuf) {
    let cred_name = args.get(2).expect("Please supply a credential name");
    let key_file_path = file::read(cred_name, folder);
    if !Path::new(&key_file_path).exists() { println!("Key does not exist at location: {}", key_file_path)};
    let clear_ssh_agent = Command::new("bash")
        .arg("-c")
        .arg("ssh-add -D".to_string())
        .output();
    match clear_ssh_agent {
        Ok(output) => {
            println!("Removed previous keys from ssh-agent cache");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        },
        Err(err) => println!("Failed to remove previous keys from ssh-agent cache {}", err),
    }
    let command = Command::new("bash")
        .arg("-c")
        .arg(&format!("ssh-add {}", key_file_path))
        .output();
    match command {
        Ok(output) => {
            println!("SSH key loaded successfully.");
            println!("{}", String::from_utf8_lossy(&output.stderr));
        },
        Err(err) => println!("Failed to execute ssh-add: {}", err),
    }
}