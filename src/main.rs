mod args;
mod configurator;
mod error;
mod profile_applicator;
mod result;
mod settings;

use std::process::ExitCode;

use anyhow::Context;
use args::{Config, ConfigKey, ProfileConfig, WorkHoursConfig};
use chrono::Duration;
use clap::Parser;
use lazy_static::lazy_static;
use settings::Override;

use crate::args::{Args, Command};
use crate::configurator::Configurator;
use crate::profile_applicator::ProfileApplicator;
use crate::result::Result;
use crate::settings::{ProfileType, Settings};

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
    let mut settings = Settings::new().context("failed to read settings")?;

    match args.command().unwrap_or(&Default::default()) {
        Command::Auto => ProfileApplicator::auto(&settings).apply(),
        Command::Restore => {
            settings.reset_override();
            ProfileApplicator::auto(&settings).apply()
        }
        Command::Work { input_duration } => {
            settings.set_override(Override::new(ProfileType::Work, input_duration.duration()));
            ProfileApplicator::new(&settings, ProfileType::Work).apply()
        }
        Command::Home { input_duration } => {
            settings.set_override(Override::new(ProfileType::Home, input_duration.duration()));
            ProfileApplicator::new(&settings, ProfileType::Home).apply()
        }
        Command::Config(config) => {
            let mut configurator = Configurator::new(&mut settings);
            match &config.config {
                Some(Config::Work(ProfileConfig { key, value })) => {
                    handle_profile_config(
                        &mut configurator,
                        ProfileType::Work,
                        key.as_ref(),
                        value.as_deref(),
                    )?;
                }
                Some(Config::Home(ProfileConfig { key, value })) => {
                    handle_profile_config(
                        &mut configurator,
                        ProfileType::Home,
                        key.as_ref(),
                        value.as_deref(),
                    )?;
                }
                Some(Config::WorkHours(WorkHoursConfig { transition, time })) => {
                    match (transition, time) {
                        (Some(transition), Some(time)) => {
                            let time = time.parse().context("time format must be hh:mm:ss")?;
                            configurator.set_clocking_time(*transition, time)
                        }
                        (Some(transition), _) => {
                            println!("{}", configurator.clocking_time(*transition));
                        }
                        _ => print!("{}", serde_yaml::to_string(configurator.clocking_times())?),
                    }
                }
                None => print!("{}", serde_yaml::to_string(configurator.settings())?),
            }
            Ok(())
        }
    }?;

    settings.save()?;

    Ok(())
}

fn handle_profile_config(
    configurator: &mut Configurator,
    profile_type: ProfileType,
    key: Option<&ConfigKey>,
    value: Option<&str>,
) -> Result<()> {
    match (key, value) {
        (Some(key), Some(value)) => configurator.set(profile_type, key, value),
        (Some(key), _) => println!("{}", configurator.get(profile_type, key)),
        _ => print!(
            "{}",
            serde_yaml::to_string(configurator.profile(profile_type))?
        ),
    }
    Ok(())
}
