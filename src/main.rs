mod args;
mod configurator;
mod profile_applicator;
mod settings;

use std::error::Error;

use clap::Parser;

use crate::args::{Args, Command, ProfileType};
use crate::configurator::Configurator;
use crate::profile_applicator::ProfileApplicator;
use crate::settings::Settings;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut settings = Settings::new()?;
    println!("{settings:#?}");

    match args.command().unwrap_or(&Default::default()) {
        Command::Auto => ProfileApplicator::auto(&settings).apply(),
        Command::Work => {
            settings.declare_profile_overridden(ProfileType::Work);
            println!("{settings:#?}");
            ProfileApplicator::new(&settings, ProfileType::Work).apply()
        }
        Command::Play => {
            settings.declare_profile_overridden(ProfileType::Play);
            println!("{settings:#?}");
            ProfileApplicator::new(&settings, ProfileType::Play).apply()
        }
        Command::Config(config) => {
            let mut configurator = Configurator::new(&mut settings);
            if let Some(value) = &config.value {
                configurator.set(&config.profile_type, &config.key, value);
            } else {
                println!("{}", configurator.get(&config.profile_type, &config.key));
            }
            Ok(())
        }
    }?;

    settings.save()?;
    Ok(())
}
