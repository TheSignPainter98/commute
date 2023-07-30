mod args;
mod profile_applier;
mod settings;

use std::error::Error;

use clap::Parser;

use crate::args::{Args, Command, Config};
use crate::profile_applier::ProfileApplier;
use crate::settings::{ProfileType, Settings};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut settings = Settings::new()?;
    println!("{settings:#?}");

    match args.command().unwrap_or(&Default::default()) {
        Command::Auto => ProfileApplier::auto(&args, &settings).apply(),
        Command::Work => ProfileApplier::new(&args, &settings, ProfileType::Work).apply(),
        Command::Play => ProfileApplier::new(&args, &settings, ProfileType::Play).apply(),
        Command::Config(cfg) => config(&args, &cfg, &mut settings),
    }?;

    settings.save()?;
    Ok(())
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
