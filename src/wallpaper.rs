/// Wallpaper management functions

use std::cmp::min;
use std::path::PathBuf;
use std::process::Command;
use timer::Timer;
use chrono::{Duration, Timelike, NaiveTime, Local};
use crate::config::Config;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

pub fn next_wallpaper(config_path: PathBuf, verbose: bool) {
    todo!()
}

pub fn prev_wallpaper(config_path: PathBuf, verbose: bool) {
    todo!()
}

pub fn show_wallpaper(wallpaper_path: PathBuf, verbose: bool) {
    let output = Command::new("feh")
        .arg("--bg-fill")
        .arg(&wallpaper_path)
        .output()
        .expect("failed to execute process");
    if verbose {
        println!("feh output: {}", output.status);
        println!("Current_wallpaper: {:?}", wallpaper_path);
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

fn get_current_time() -> NaiveTime {
    let offset_in_seconds = Local::now().offset().local_minus_utc();
    let now = Local::now().naive_local() - Duration::seconds(offset_in_seconds.into());
    now.time()
}

fn get_wallpaper_index(wallpaper_paths: Vec<PathBuf>, hour: usize) -> usize {
    let file_names: Vec<&std::ffi::OsStr> = wallpaper_paths.iter().map(|x| x.file_stem().unwrap()).collect();
    let file_names: Vec<&str> = file_names.iter().map(|x| x.to_str().unwrap()).collect();
    let index = file_names.iter()
        .position(|x| x.parse::<usize>().unwrap() == hour)
        .unwrap_or_else(|| {
            let mut index = 0;
            let mut min_diff = 24;
            for (i, name) in file_names.iter().enumerate() {
                let diff = min((name.parse::<usize>().unwrap() as i32 - hour as i32).abs(), (hour as i32 - name.parse::<usize>().unwrap() as i32).abs());
                if diff < min_diff {
                    min_diff = diff;
                    index = i;
                }
            }
            index
        });
    index
}

/// Cyle through wallpapers in folder based on time of day
pub fn cycle_wallpaper_versions(wallpaper_path: PathBuf, verbose: bool) {
    let timer = Timer::new();
    let now = get_current_time().hour() as usize;
    let wallpaper_paths: Vec<PathBuf> = get_wallpaper_versions(wallpaper_path.clone(), verbose);
    let current_wallpaper = wallpaper_paths[get_wallpaper_index(wallpaper_paths.clone(), now)].clone();
    show_wallpaper(current_wallpaper.clone(), verbose);
    let data = Arc::new(Mutex::new(0));
    let (_tx, rx) = channel::<i32>();
    let _guard = timer.schedule_repeating(Duration::seconds(10), move || {
        let wallpaper_paths: Vec<PathBuf> = get_wallpaper_versions(wallpaper_path.clone(), verbose);
        let now = get_current_time();
        if verbose {
            println!("Current time: {}", now);
        }
        let now = now.hour() as usize;
        if verbose {
            println!("Current hour: {}", now);
        }

        let current_wallpaper = wallpaper_paths[get_wallpaper_index(wallpaper_paths.clone(), now)].clone();

        
        show_wallpaper(current_wallpaper.clone(), verbose);
        let mut _data = data.lock().unwrap();
    });
    rx.recv().unwrap();
}

/// Select wallpaper folder and dynamically change wallpapers
/// based on time of day 
pub fn select_and_start(selected_folder: PathBuf, config_path: PathBuf, verbose: bool) {
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let wallpaper_folders = configuration.wallpaper_folders;
    let matcher = SkimMatcherV2::default();
    let mut fuzzy_matches: Vec<(i64, PathBuf)> = Vec::new();
    for folder in wallpaper_folders {
        let score = matcher.fuzzy_match(selected_folder
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap(), 
                                        folder
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()).unwrap_or(0.into());
        fuzzy_matches.push((score, folder));
    }
    for x in &fuzzy_matches {
        if verbose {
            println!("Fuzzy match: {:?}", x);
        }
    }
    fuzzy_matches.sort_by(|a, b| b.0.cmp(&a.0));
    if &fuzzy_matches[0].0 < &50 {
        println!("No matches found");
        std::process::exit(1);
    } else {
        let selected_folder = &fuzzy_matches[0].1;
    
        if verbose {
             println!("Selected folder: {:?}", selected_folder);
        }
        cycle_wallpaper_versions(selected_folder.clone(), verbose);
    }
}

pub fn start(config_path: PathBuf, verbose: bool) {
    todo!()
}
