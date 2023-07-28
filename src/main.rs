mod args;
mod settings;

use std::error::Error;

use clap::Parser;

use crate::args::{Args, Command, Config};
use crate::settings::Settings;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut settings = Settings::new()?;
    println!("{settings:#?}");

    match args.command().unwrap_or(&Default::default()) {
        Command::Auto => auto(&args),
        Command::Work => work(&args),
        Command::Play => play(&args),
        Command::Config(cfg) => config(&args, &cfg, &mut settings),
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

fn config(_args: &Args, cfg: &Config, settings: &mut Settings) -> Result<(), Box<dyn Error>> {
    macro_rules! handle_config {
        ($field:expr, $value:ident) => {
            match $value {
                None => println!("{}", $field),
                Some(value) => $field = value.clone(),
            }
        };
    }

    match cfg {
        Config::WorkBrowser { browser } => handle_config!(settings.work.browser, browser),
        Config::PlayBrowser { browser } => handle_config!(settings.play.browser, browser),
        Config::WorkBackgroundDir { dir } => handle_config!(settings.work.background_dir, dir),
        Config::PlayBackgroundDir { dir } => handle_config!(settings.play.background_dir, dir),
    }
    Ok(())
}
