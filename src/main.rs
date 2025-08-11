mod config;
mod file;

use std::{env, process};
use dirs::home_dir;
use std::path::{PathBuf};

struct Config {
    config_folder: PathBuf,
}

fn print_help() {
    println!("No parameter supplied");
    println!("Usage:");
    println!("- save (name) (key_locations_path)");
    println!("- load (name)");
    println!("- delete (name)");
    println!("- list");
}

fn main() {
    let config = Config {
        config_folder: home_dir().unwrap().join(".config/gitkeyswitcher/credconfigs")
    };

    let args: Vec<String> = env::args().collect();

    let super_parameter = match args.get(1) {
        Some(param) => param,
        None => {
            print_help();
            process::exit(1);
        }
    };

    if super_parameter == "save" {
        config::save(args, config.config_folder);
    } else if super_parameter == "load" {
        config::load(args, config.config_folder);
    } else if super_parameter == "delete" {
        let cred_name = args.get(2).expect("Please supply a credential name");
        file::delete(cred_name, config.config_folder)
    } else if super_parameter == "list" {
        file::list(config.config_folder);
    } else {
        println!("Invalid parameter");
    };
}