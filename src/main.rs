/// Hitsuki is a commandline tool for dynamic time based wallpapers in x11 
/// based window managers. It is written in rust and uses the feh to 
/// interact with the x11 server and set the background.
use std::path::PathBuf;
use path_dedot::*;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

use crate::process::daemonize;
use crate::process::match_commands;

pub mod config;
pub mod wallpaper;
pub mod process;

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
pub struct Cli {
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

    /// Restart daemonized processes
    #[arg(short, long)]
    restart: bool,
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

    /// Kill all hitsuki processes
    Kill,

    /// Show status and if daemonized
    Status,
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

    if cli.restart {
        process::restart_daemon(verbose);
    }

    if cli.daemonize {
        match daemonize(&cli, config_path.clone(), verbose) {
            Ok(_) => {}
            Err(_) => {
            println!("Daemon already running...");
            }
        };
    } else {
        match_commands(&cli, config_path, verbose);
    } 
}
