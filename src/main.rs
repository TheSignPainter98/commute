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
        Command::Work => {
            settings.declare_profile_overridden(ProfileType::Work);
            ProfileApplier::new(&args, &settings, ProfileType::Work).apply()
        }
        Command::Play => {
            settings.declare_profile_overridden(ProfileType::Play);
            ProfileApplier::new(&args, &settings, ProfileType::Play).apply()
        }
        Command::Config(cfg) => config(&args, &cfg, &mut settings),
    }?;

    settings.save()?;
    Ok(())
}

fn config(_args: &Args, cfg: &Config, settings: &mut Settings) -> Result<(), Box<dyn Error>> {
    match cfg {
        Config::WorkBrowser { browser } => match browser {
            None => println!("{}", settings.work.browser()),
            Some(browser) => settings.work.set_browser(browser.clone()),
        },
        Config::PlayBrowser { browser } => match browser {
            None => println!("{}", settings.play.browser()),
            Some(browser) => settings.play.set_browser(browser.clone()),
        },
        Config::WorkBackgroundDir { dir } => match dir {
            None => println!("{}", settings.work.background_dir()),
            Some(dir) => settings.work.set_background_dir(dir.clone()),
        },
        Config::PlayBackgroundDir { dir } => match dir {
            None => println!("{}", settings.play.background_dir()),
            Some(dir) => settings.play.set_background_dir(dir.clone()),
        },
    }
    Ok(())
}
