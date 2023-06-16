/// Configuration file functions
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use confy;

#[derive(Serialize, Deserialize)]
struct Config {
    wallpaper_folders: Vec<PathBuf>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            wallpaper_folders: vec![],
        }
    }
}

pub fn check_config(config_path: PathBuf, verbose: bool) -> Option<PathBuf> {
    let _configuration: Config = confy::load_path(&config_path).unwrap();
    if verbose {
        println!("Config path: {:?}", &config_path);
    }
    Some(config_path)
}

pub fn add_folder(addpath: PathBuf, config_path: PathBuf) {
    let mut configuration: Config = confy::load_path(&config_path).unwrap();
    configuration.wallpaper_folders.push(addpath);
    confy::store_path(&config_path, configuration).unwrap();
}

pub fn remove_folder(rmpath: PathBuf, config_path: PathBuf) {
    let mut configuration: Config = confy::load_path(&config_path).unwrap();
    let index = configuration.wallpaper_folders.iter().position(|x| *x == rmpath).unwrap();
    configuration.wallpaper_folders.remove(index);
    confy::store_path(&config_path, configuration).unwrap();
}

pub fn list_folders(config_path: PathBuf) {
    let configuration: Config = confy::load_path(&config_path).unwrap();
    for folder in configuration.wallpaper_folders {
        println!("{:?}", folder);
    }
}

