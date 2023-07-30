mod args;
mod configurator;
mod profile_applier;
mod settings;

use std::error::Error;

use clap::Parser;

use crate::args::{Args, Command};
use crate::configurator::Configurator;
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
            println!("{settings:#?}");
            ProfileApplier::new(&args, &settings, ProfileType::Work).apply()
        }
        Command::Play => {
            settings.declare_profile_overridden(ProfileType::Play);
            println!("{settings:#?}");
            ProfileApplier::new(&args, &settings, ProfileType::Play).apply()
        }
        Command::Config(config_option) => {
            // TODO(kcza): make the config_option be either Show(field) |
            // Set(field, value)
            Configurator::new(&args, &mut settings).config(config_option)
        }
    }?;

    settings.save()?;
    Ok(())
}
