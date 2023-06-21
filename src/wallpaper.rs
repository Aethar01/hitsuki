/// Wallpaper management functions

// use std::env;
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
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let current_wallpaper_index = configuration.current_wallpaper_index;
    let wallpaper_folders = configuration.wallpaper_folders;
    let next_wallpaper_index = if (current_wallpaper_index + 1) >= wallpaper_folders.len() {
        0
    } else {
        current_wallpaper_index + 1
    };
    // let hour = get_current_time().hour() as usize;
    let next_wallpaper_buf = wallpaper_folders[next_wallpaper_index].clone();
    // let next_wallpaper_buf_image = get_wallpaper_index(get_wallpaper_versions(&next_wallpaper_buf.clone(), verbose), hour);
    // let next_wallpaper_versions = get_wallpaper_versions(&next_wallpaper_buf, verbose);
    // let next_wallpaper_buf_image = next_wallpaper_versions.iter().collect::<Vec<_>>()[next_wallpaper_buf_image];
    // let current_wallpaper_buf = wallpaper_folders[current_wallpaper_index].clone();
    // let current_wallpaper_buf_image = get_wallpaper_index(get_wallpaper_versions(&current_wallpaper_buf.clone(), verbose), hour);
    // let current_wallpaper_versions = get_wallpaper_versions(&current_wallpaper_buf, verbose);
    // let current_wallpaper_buf_image = current_wallpaper_versions.iter().collect::<Vec<_>>()[current_wallpaper_buf_image];

    if verbose {
        println!("Next wallpaper: {:?}", next_wallpaper_buf);
    }
    // let transition_wallpaper = make_transition_wallpaper(current_wallpaper_buf_image.to_path_buf(), next_wallpaper_buf_image.to_path_buf(), verbose);
    // show_wallpaper(transition_wallpaper, verbose);
    let mut configuration: Config = confy::load_path(&config_path).unwrap();
    configuration.current_wallpaper_index = next_wallpaper_index;
    confy::store_path(&config_path, configuration).unwrap();
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let wallpaper_folders = configuration.wallpaper_folders;
    let wallpaper_folder = wallpaper_folders[configuration.current_wallpaper_index].clone();
    cycle_wallpaper_versions(wallpaper_folder, verbose)
}

pub fn prev_wallpaper(config_path: PathBuf, verbose: bool) {
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let current_wallpaper_index = configuration.current_wallpaper_index;
    let wallpaper_folders = configuration.wallpaper_folders;
    let prev_wallpaper_index = if current_wallpaper_index == 0 {
        wallpaper_folders.len() - 1
    } else {
        current_wallpaper_index - 1
    };
    let prev_wallpaper_buf = wallpaper_folders[prev_wallpaper_index].clone();
    // let current_wallpaper_buf = wallpaper_folders[current_wallpaper_index].clone();
    if verbose {
        println!("Previous wallpaper: {:?}", prev_wallpaper_buf);
    }
    // let transition_wallpaper = make_transition_wallpaper(current_wallpaper_buf, prev_wallpaper_buf, verbose);
    // show_wallpaper(transition_wallpaper, verbose);
    let mut configuration: Config = confy::load_path(&config_path).unwrap();
    configuration.current_wallpaper_index = prev_wallpaper_index;
    confy::store_path(&config_path, configuration).unwrap();
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let wallpaper_folders = configuration.wallpaper_folders;
    let wallpaper_folder = wallpaper_folders[configuration.current_wallpaper_index].clone();
    cycle_wallpaper_versions(wallpaper_folder, verbose)
}

// fn make_transition_wallpaper(wall1: PathBuf, wall2: PathBuf, verbose: bool) -> PathBuf {
//     let tmp_path = env::temp_dir().join("transition_wallpaper.png");
//     if tmp_path.exists() {
//         std::fs::remove_file(tmp_path.clone()).unwrap();
//     }
//     println!("{}, {}", wall1.display(), wall2.display());
//     let output = Command::new("convert")
//         .arg(&wall1)
//         .arg(&wall2)
//         .arg("-average")
//         .arg(&tmp_path)
//         .output()
//         .expect("failed to execute process");
//     if !output.status.success() {
//         println!("convert error: {}", String::from_utf8_lossy(&output.stderr));
//     }
//     if verbose {
//         println!("convert output: {}", output.status);
//         println!("Transition_wallpaper: {:?}", tmp_path);
//     }
//     tmp_path
// }

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

fn get_wallpaper_versions(wallpaper_path: &PathBuf, verbose: bool) -> Vec<PathBuf> {
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

/// Returns current time
fn get_current_time() -> NaiveTime {
    let offset_in_seconds = Local::now().offset().local_minus_utc();
    let now = Local::now().naive_local() - Duration::seconds(offset_in_seconds.into());
    now.time()
}

/// Returns index of wallpaper to be shown based on time of day
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
    let wallpaper_paths: Vec<PathBuf> = get_wallpaper_versions(&wallpaper_path, verbose);
    let current_wallpaper = wallpaper_paths[get_wallpaper_index(wallpaper_paths.clone(), now)].clone();
    show_wallpaper(current_wallpaper.clone(), verbose);
    let data = Arc::new(Mutex::new(0));
    let (_tx, rx) = channel::<i32>();
    let _guard = timer.schedule_repeating(Duration::seconds(60), move || {
        let wallpaper_paths: Vec<PathBuf> = get_wallpaper_versions(&wallpaper_path, verbose);
        let now = get_current_time();
        if verbose {
            println!("Current time: {}", now);
        }
        let now = now.hour() as usize;
        if verbose {
            println!("Current hour: {}", now);
        }

        // for _ in 1..=5 {
        //     let old_wallpaper = wallpaper_paths[get_wallpaper_index(wallpaper_paths.clone(), now - 1)].clone();
        //     let new_wallpaper = wallpaper_paths[get_wallpaper_index(wallpaper_paths.clone(), now)].clone();
        //     let transition_wallpaper = make_transition_wallpaper(old_wallpaper, new_wallpaper, verbose);
        //     show_wallpaper(transition_wallpaper.clone(), verbose);
        // }

        
        let current_wallpaper = wallpaper_paths[get_wallpaper_index(wallpaper_paths.clone(), now)].clone();
        show_wallpaper(current_wallpaper.clone(), verbose);
        let mut _data = data.lock().unwrap();
    });
    rx.recv().unwrap();
}

/// Returns fuzzy match score between two PathBufs
fn fuzzy_match_path_buf(a: &PathBuf, b: &PathBuf) -> i64 {
    let matcher = SkimMatcherV2::default();
    let score = matcher.fuzzy_match(a
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(), 
                        b
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()).unwrap_or(0.into());
    score
}

/// Select wallpaper folder and dynamically change wallpapers
/// based on time of day 
pub fn select_and_start(selected_folder: PathBuf, config_path: PathBuf, verbose: bool) {
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let wallpaper_folders = configuration.wallpaper_folders;
    let mut fuzzy_matches: Vec<(i64, PathBuf)> = Vec::new();
    for folder in &wallpaper_folders {
        let score = fuzzy_match_path_buf(&selected_folder, &folder);
        fuzzy_matches.push((score, folder.to_path_buf()));
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
        let index = wallpaper_folders.iter().position(|x| x == selected_folder).unwrap();
        crate::config::set_current_wallpaper(config_path, index);
        cycle_wallpaper_versions(selected_folder.clone(), verbose);
    }
}

pub fn set_wallpaper(selected_folder: PathBuf, config_path: PathBuf, verbose: bool) {
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let wallpaper_folders = configuration.wallpaper_folders;
    let index = wallpaper_folders.iter().position(|x| x == &selected_folder).unwrap();
    if verbose {
        println!("Selected folder: {:?}", selected_folder);
    }
    crate::config::set_current_wallpaper(config_path, index);
}

pub fn start(config_path: PathBuf, verbose: bool) {
    let configuration: Config = confy::load_path(&config_path).unwrap();
    let wallpaper_folders = configuration.wallpaper_folders;
    let current_wallpaper = wallpaper_folders[configuration.current_wallpaper_index].clone();
    cycle_wallpaper_versions(current_wallpaper, verbose);
}
