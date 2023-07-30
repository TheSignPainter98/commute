mod args;
mod settings;

use std::error::Error;

use chrono::{Datelike, NaiveTime, Utc};
use clap::Parser;
use lazy_static::lazy_static;
use settings::Override;

use crate::args::{Args, Command, Config};
use crate::settings::{ProfileType, Settings};

lazy_static! {
    static ref WORK_START: NaiveTime = NaiveTime::from_hms_opt(6, 0, 0).unwrap();
    static ref WORK_END: NaiveTime = NaiveTime::from_hms_opt(18, 30, 00).unwrap();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut settings = Settings::new()?;
    println!("{settings:#?}");

    match args.command().unwrap_or(&Default::default()) {
        Command::Auto => auto(&args, &settings),
        Command::Work => work(&args),
        Command::Play => play(&args),
        Command::Config(cfg) => config(&args, &cfg, &mut settings),
    }?;

    settings.save()?;
    Ok(())
}

fn auto(args: &Args, settings: &Settings) -> Result<(), Box<dyn Error>> {
    let profile_type = settings
        .r#override()
        .map(Override::advise_profile)
        .flatten()
        .unwrap_or_else(|| {
            let now = Utc::now();
            use chrono::Weekday::*;
            match now.weekday() {
                Sat | Sun => ProfileType::Play,
                _ => {
                    let time = now.time();
                    if *WORK_START <= time && time <= *WORK_END {
                        ProfileType::Work
                    } else {
                        ProfileType::Play
                    }
                }
            }
        });
    match profile_type {
        ProfileType::Work => work(args),
        ProfileType::Play => play(args),
    }
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
