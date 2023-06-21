/// Configuration file functions
use std::path::PathBuf;
use path_dedot::ParseDot;
use serde::{Deserialize, Serialize};
use confy;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub wallpaper_folders: Vec<PathBuf>,
    pub current_wallpaper_index: usize,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            wallpaper_folders: vec![],
            current_wallpaper_index: 0,
        }
    }
}

pub fn check_config(config_path: PathBuf, verbose: bool) -> Option<PathBuf> {
    if !config_path.is_file() {
        println!("{} is not a file", config_path.display());
        std::process::exit(1);
    }
    let _configuration: Config = confy::load_path(&config_path).unwrap();
    if verbose {
        println!("Config path: {:?}", &config_path);
    }
    Some(config_path)
}

pub fn add_folder(addpath: PathBuf, config_path: PathBuf) {
    if !addpath.is_dir() {
        println!("{} is not a directory", addpath.display());
        std::process::exit(1);
    }
    let addpath = addpath.parse_dot().unwrap().to_path_buf();
    let mut configuration: Config = confy::load_path(&config_path).unwrap();
    if !configuration.wallpaper_folders.contains(&addpath) {
        configuration.wallpaper_folders.push(addpath);
        confy::store_path(&config_path, configuration).unwrap();
    } else {
        println!("{} is already in the config file", addpath.display());
    }
}

pub fn remove_folder(rmpath: PathBuf, config_path: PathBuf) {
    if !rmpath.is_dir() {
        println!("{} is not a directory", rmpath.display());
        std::process::exit(1);
    }
    let rmpath = rmpath.parse_dot().unwrap().to_path_buf();
    let mut configuration: Config = confy::load_path(&config_path).unwrap();
    let index = configuration.wallpaper_folders.iter().position(|x| *x == rmpath).unwrap();
    if configuration.wallpaper_folders.contains(&rmpath) {
        configuration.wallpaper_folders.remove(index);
        confy::store_path(&config_path, configuration).unwrap();
    } else {
        println!("{} is not in the config file", rmpath.display());
    }
}

pub fn list_folders(config_path: PathBuf) {
    if !config_path.is_file() {
        println!("{} is not a file", config_path.display());
        std::process::exit(1);
    }
    let configuration: Config = confy::load_path(&config_path).unwrap();
    for folder in configuration.wallpaper_folders {
        println!("{:?}: {:?}", folder, folder.file_stem().unwrap());
    }
}

pub fn set_current_wallpaper(config_path: PathBuf, index: usize) {
    let mut configuration: Config = confy::load_path(&config_path).unwrap();
    configuration.current_wallpaper_index = index;
    confy::store_path(&config_path, configuration).unwrap();
}
