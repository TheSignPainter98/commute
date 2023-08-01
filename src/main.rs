mod args;
mod configurator;
mod error;
mod profile_applicator;
mod result;
mod settings;

use std::process::ExitCode;

use chrono::Duration;
use clap::Parser;
use lazy_static::lazy_static;
use settings::Override;

use crate::args::{Args, Command, ProfileType};
use crate::configurator::Configurator;
use crate::profile_applicator::ProfileApplicator;
use crate::result::Result;
use crate::settings::Settings;

lazy_static! {
    static ref DAY_OVERRIDE_DURATION: Duration = Duration::hours(12);
}

fn main() -> ExitCode {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run(args: Args) -> Result<()> {
    let mut settings = Settings::new()?;

    match args.command().unwrap_or(&Default::default()) {
        Command::Auto => ProfileApplicator::auto(&settings).apply(),
        Command::Work => {
            settings.set_override(Override::new(ProfileType::Work, *DAY_OVERRIDE_DURATION));
            ProfileApplicator::new(&settings, ProfileType::Work).apply()
        }
        Command::Home => {
            settings.set_override(Override::new(ProfileType::Home, *DAY_OVERRIDE_DURATION));
            ProfileApplicator::new(&settings, ProfileType::Home).apply()
        }
        Command::Away(length) => {
            settings.set_override(Override::new(ProfileType::Home, length.duration()));
            ProfileApplicator::new(&settings, ProfileType::Home).apply()
        }
        Command::Config(config) => {
            let mut configurator = Configurator::new(&mut settings);
            match (&config.profile_type, &config.key, &config.value) {
                (Some(profile_type), Some(key), Some(value)) => {
                    configurator.set(profile_type, key, value)
                }
                (Some(profile_type), Some(key), _) => {
                    println!("{}", configurator.get(profile_type, key))
                }
                (Some(profile_type), ..) => {
                    print!(
                        "{}",
                        serde_yaml::to_string(configurator.profile(profile_type))?
                    )
                }
                (None, ..) => print!("{}", serde_yaml::to_string(configurator.settings())?),
            }
            Ok(())
        }
    }?;

    settings.save()?;

    Ok(())
}
