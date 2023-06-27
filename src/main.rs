/// Hitsuki is a commandline tool for dynamic time based wallpapers in x11 
/// based window managers. It is written in rust and uses the feh to 
/// interact with the x11 server and set the background.
use std::path::PathBuf;
use path_dedot::*;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use sysinfo::{System, SystemExt, ProcessExt, Pid};
use std::fs::File;
use daemonize::Daemonize;

pub mod config;
pub mod wallpaper;

lazy_static! {
    static ref DEFAULT_CONFIG_PATH: PathBuf = {
        let mut path = dirs::config_dir().unwrap();
        path.push("hitsuki");
        path.push("config.toml");
        path
    };
}

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    /// Sets a custom config file.
    /// Default: $HOME/.config/hitsuki/config.toml
    #[arg(short, long, value_name = "FILE", default_value = DEFAULT_CONFIG_PATH.to_str().unwrap())]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,

    /// List all wallpaper folders
    #[arg(short, long)]
    list: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long, value_name = "FOLDER NAME")]
    set: Option<PathBuf>,

    /// Run as daemon
    #[arg(short, long)]
    daemonize: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Add new wallpaper folder
    Add {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },

    /// Remove wallpaper folder
    Remove {
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },

    /// Show next wallpaper
    Next,

    /// Show previous wallpaper
    Prev,

    /// Select wallpaper folder 
    /// and dynamically change wallpaper
    /// based on time of day
    SelectandStart {
        #[arg(value_name = "WALLPAPER_NAME")]
        folder_name: PathBuf,
    },

    /// Start dynamically changing wallpapers
    /// based on time of day
    /// from your added wallpaper folders
    Start, 

    /// Select wallpaper folder
    /// and set config to selected wallpaper
    Set {
        #[arg(value_name = "WALLPAPER_NAME")]
        folder_name: PathBuf,
    },

    /// Stop daemonized processes
    StopDaemon,
}

fn ps_pids(verbose: bool) -> Vec<Pid> {
    let mut system = System::new();
    system.refresh_all();
    let ps = system.processes_by_name("hitsuki");
    let ps_pids = ps.map(|p| p.pid()).collect::<Vec<_>>();
    if verbose {
        println!("PIDS: {:?}", ps_pids);
    }
    ps_pids
}

fn kill_other_instances(verbose: bool) {
    let mut system = System::new();
    system.refresh_all();
    let ps_pids = ps_pids(verbose);
    let this_pid = Pid::from(std::process::id() as usize);
    for pid in ps_pids {
        if pid != this_pid {
            if let Some(process) = system.process(pid) {
                process.kill();
            }
        }
    }
}


// fn check_daemonized(verbose: bool) -> bool {
//     let ps_pids = ps_pids(verbose);
//     let run = dirs::runtime_dir().unwrap();
//     let run = run.to_str().unwrap();
//     let pid_file = format!("{}/hitsuki/hitsuki.pid", run);
//     let daemon_pid = std::fs::read_to_string(pid_file).unwrap().trim().parse::<usize>().unwrap();
//     for pid in ps_pids {
//         if pid == Pid::from(daemon_pid) && pid != Pid::from(std::process::id() as usize) {
//             true;
//         }
//     }
//     false
// }

fn stop_daemon(verbose: bool) {
    let run = dirs::runtime_dir().unwrap();
    let run = run.to_str().unwrap();
    let pid_file = format!("{}/hitsuki/hitsuki.pid", run);
    let daemon_pid = std::fs::read_to_string(pid_file).unwrap().trim().parse::<usize>().unwrap();
    let mut system = System::new();
    system.refresh_all();
    if verbose {
        println!("Killing daemon with pid: {}", daemon_pid);
    }
    if let Some(process) = system.process(Pid::from(daemon_pid)) {
        process.kill();
    }
}

fn match_commands(cli: Cli, config_path: PathBuf, verbose: bool) {
    match &cli.command {
        Some(Commands::Add { path }) => {
            config::add_folder(path.clone(), config_path);
        }
        Some(Commands::Remove { path }) => {
            config::remove_folder(path.clone(), config_path);
        }
        Some(Commands::Next) => {
            wallpaper::next_wallpaper(config_path, verbose);
        }
        Some(Commands::Prev) => {
            wallpaper::prev_wallpaper(config_path, verbose);
        }
        Some(Commands::SelectandStart { folder_name }) => {
            wallpaper::select_and_start(folder_name.clone(), config_path, verbose);
        }
        Some(Commands::Start) => {
            wallpaper::start(config_path, verbose);
        }
        Some(Commands::Set { folder_name }) => {
            wallpaper::set_wallpaper(folder_name.clone(), config_path, verbose);
        }
        Some(Commands::StopDaemon) => {
            stop_daemon(verbose);
        }
        None => {
            config::check_config(config_path, verbose);
        }
    }
}

fn daemonize(cli: Cli, config_path: PathBuf, verbose: bool) {
        let run = dirs::runtime_dir().unwrap();
        let run = run.to_str().unwrap();
        std::fs::create_dir_all(format!("{}/hitsuki", run)).unwrap();
        let stdout = File::create(format!("{}/hitsuki/hitsuki.out", run)).unwrap();
        let stderr = File::create(format!("{}/hitsuki/hitsuki.err", run)).unwrap();
        let daemonize = Daemonize::new()
            .pid_file(format!("{}/hitsuki/hitsuki.pid", run))
            .stdout(stdout)
            .stderr(stderr);
        match daemonize.start() {
            Ok(_) => {
                match_commands(cli, config_path, verbose);
            }
            Err(e) => {
                if verbose {
                    println!("Error, {}", e);
                }
                println!("Daemon already running")
            }
        }
}

fn main() {
    let cli = Cli::parse();
    let config_path = cli.config.as_ref().unwrap().parse_dot().unwrap().to_path_buf();
    let verbose = cli.verbose;

    if cli.set.is_some() {
        wallpaper::set_wallpaper(cli.set.clone().unwrap(), config_path.clone(), verbose);
    }

    if cli.list {
        config::list_folders(config_path.clone());
    }

    if cli.daemonize {
        daemonize(cli, config_path, verbose);
    } else {
        match_commands(cli, config_path, verbose);
    } 

}
