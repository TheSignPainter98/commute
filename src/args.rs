use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use serde::{Deserialize as Deserialise, Serialize as Serialise};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[warn(missing_docs)]
pub(crate) struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Args {
    pub(crate) fn command(&self) -> Option<&Command> {
        self.command.as_ref()
    }
}

#[derive(Subcommand, Debug, PartialEq, Eq, Default)]
#[warn(missing_docs)]
pub(crate) enum Command {
    /// Automatically
    #[default]
    Auto,

    /// Set play presets
    Play,

    /// Set work presets
    Work,

    /// Change configuration
    Config(Config),
}

#[derive(ClapArgs, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) struct Config {
    /// The profile to query
    #[clap(name = "profile")]
    pub(crate) profile_type: ProfileType,

    /// The setting in the profile to query
    #[clap(name = "setting")]
    pub(crate) key: ConfigKey,

    /// If present, set the specified setting to this value, otherwise print it
    #[clap(name = "value")]
    pub(crate) value: Option<String>,
}

#[derive(ValueEnum, Copy, Clone, Debug, Serialise, Deserialise, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) enum ProfileType {
    Work,
    Play,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) enum ConfigKey {
    Browser,
    BackgroundDir,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn auto() {
        assert_eq!(None, Args::parse_from(["levo"]).command());
        assert_eq!(
            &Command::Auto,
            Args::parse_from(["levo", "auto"]).command().unwrap()
        );
    }

    #[test]
    fn play() {
        assert_eq!(
            &Command::Play,
            Args::parse_from(["levo", "play"]).command().unwrap()
        );
    }

    #[test]
    fn work() {
        assert_eq!(
            &Command::Work,
            Args::parse_from(["levo", "work"]).command().unwrap()
        );
    }

    // #[test]
    // fn set() {
    //     assert!(matches!(
    //         Args::parse_from(&["em", "set", "work-browser", "/usr/bin/internet-explorer-6"])
    //             .command().unwrap(),
    //         Command::Set(Setting::WorkBrowser { browser if browser != "" })
    //     ));
    // }
}
