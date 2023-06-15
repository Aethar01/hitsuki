// Hitsuki is a commandline tool for dynamic time based wallpapers in x11 based
// window managers. It is written in rust and uses the x11 crate to interact
// with the x11 server.


use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "0.1.0", author = "Elliott A. <elliott.ashby88@gmail.com>")]
struct Args {
}



fn main() {
    println!("Hello, world!");
}
