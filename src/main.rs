mod args;
mod configurator;
mod error;
mod profile_applicator;
mod result;
mod settings;

use std::process::ExitCode;

use clap::Parser;

use crate::args::{Args, Command, ProfileType};
use crate::configurator::Configurator;
use crate::profile_applicator::ProfileApplicator;
use crate::result::Result;
use crate::settings::Settings;

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
            settings.declare_profile_overridden(ProfileType::Work);
            ProfileApplicator::new(&settings, ProfileType::Work).apply()
        }
        Command::Play => {
            settings.declare_profile_overridden(ProfileType::Play);
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
