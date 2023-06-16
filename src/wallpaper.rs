/// Wallpaper management functions

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, SystemTime};
use timer::Timer;
use toml::value::Datetime;

pub fn next_wallpaper(config_path: PathBuf, verbose: bool) {
}

pub fn prev_wallpaper(config_path: PathBuf, verbose: bool) {
}

pub fn show_wallpaper(wallpaper_path: PathBuf, verbose: bool) {
    let output = Command::new("feh")
        .arg("--bg-fill")
        .arg(wallpaper_path)
        .output()
        .expect("failed to execute process");
    if verbose {
        println!("feh output: {}", output.status);
    }
    if !output.status.success() {
        println!("feh error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn get_wallpaper_versions(wallpaper_path: PathBuf, verbose: bool) -> Vec<PathBuf> {
    let paths = std::fs::read_dir(wallpaper_path).unwrap();
    let mut wallpaper_paths: Vec<PathBuf> = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            if verbose {
                println!("Found wallpaper: {:?}", path);
            }
            wallpaper_paths.push(path);
        }
    }
    wallpaper_paths
}

/// Cyle through wallpapers in folder based on time of day
pub fn cycle_wallpaper_versions(wallpaper_path: PathBuf, verbose: bool) {
    let wallpaper_paths: Vec<PathBuf> = get_wallpaper_versions(wallpaper_path, verbose);
    let mut current_wallpaper: PathBuf = PathBuf::new();
    let timer = Timer::new();
    let _guard = timer.schedule_with_date();
}

