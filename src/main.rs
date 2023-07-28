mod args;
mod settings;

use std::error::Error;

use clap::Parser;

use crate::args::{Args, Command};
use crate::settings::Settings;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut settings = Settings::new()?;
    println!("{settings:#?}");

    match args.command().unwrap_or(&Default::default()) {
        Command::Auto => auto(&args),
        Command::Work => work(&args),
        Command::Play => play(&args),
        Command::Config(_) => config(&args, &mut settings),
    }?;

    settings.save()?;
    Ok(())
}

fn auto(_args: &Args) -> Result<(), Box<dyn Error>> {
    todo!("auto");
}

fn work(_args: &Args) -> Result<(), Box<dyn Error>> {
    todo!("work");
}

fn play(_args: &Args) -> Result<(), Box<dyn Error>> {
    todo!("play");
}

fn config(_args: &Args, _cfg: &mut Settings) -> Result<(), Box<dyn Error>> {
    todo!("config");
}
