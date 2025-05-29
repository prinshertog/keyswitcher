use std::env;
use std::fs::{File, create_dir_all};
use std::io::Write;
use dirs::home_dir;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let super_parameter = args.get(1).expect("No parameter supplied\n\
                                                                Usage:\n\
                                                                    - save (name) (key_locations_path)");
    if super_parameter == "save" {
        write_config_to_file(args);
    } else {
        println!("Invalid parameter")
    }
}

fn write_config_to_file(args: Vec<String>) {
    let cred_name = args.get(2).cloned().expect("Please supply a credential name");
    let key_location = args.get(3).expect("Please supply a key location");

    // Creation of the folder
    let mut folder = home_dir().unwrap();
    folder.push(".config/gitkeyswitcher/credconfigs");
    let path = Path::new(&folder);
    
    if !path.exists() {
        create_dir_all(&folder).expect("Could not create folder");
    } else {
        println!("INFO: \"credconfigs\" folder already exists");
    }


    // Creation of the config file
    let file = format!("{}/{}", folder.display(), cred_name);
    let path = Path::new(&file);
    if !path.exists() {
        let mut config_file = File::create(file).expect(&format!("Error creating config file for {}", cred_name));
        let content = format!("\
        name: {}\n\
        key_location: {}\n", cred_name, key_location);
        config_file.write_all(content.as_bytes()).expect(&format!("Error writing to config file for {}", cred_name));
        println!("Config file written to {}!", cred_name);
    } else {
        println!("ERROR: \"configfile\" already exists");
    }
}