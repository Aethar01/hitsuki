// Process management

use std::fs::File;
use sysinfo::{Pid, System, SystemExt, ProcessExt};
use daemonize::Daemonize;
use std::path::PathBuf;

use crate::config;
use crate::wallpaper;
use crate::Cli;
use crate::Commands;


pub fn match_commands(cli: &Cli, config_path: PathBuf, verbose: bool) {
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
        Some(Commands::Kill) => {
            kill_other_instances(verbose);
        }
        None => {
            config::check_config(config_path, verbose);
        }
    }
}

pub fn ps_pids(verbose: bool) -> Vec<Pid> {
    let mut system = System::new();
    system.refresh_all();
    let ps = system.processes_by_name("hitsuki");
    let ps_pids = ps.map(|p| p.pid()).collect::<Vec<_>>();
    if verbose {
        println!("PIDS: {:?}", ps_pids);
    }
    ps_pids
}

pub fn kill_other_instances(verbose: bool) {
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

pub fn stop_daemon(verbose: bool) {
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


pub fn daemonize(cli: &Cli, config_path: PathBuf, verbose: bool) -> Result<(), ()> {
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
            Ok(())
        }
        Err(e) => {
            if verbose {
                println!("Error, {}", e);
            }
            Err(())
        }
    }
}
