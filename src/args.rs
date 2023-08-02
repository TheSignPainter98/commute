use chrono::Duration;
use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};

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

    /// Set home presets
    Home,

    /// Set work presets
    Work,

    /// Set home presets for a while
    Away(AwayLength),

    /// Change configuration
    Config(ConfigCmd),
}

#[derive(ClapArgs, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) struct AwayLength {
    number: u32,
    unit: AwayLengthUnit,
}

impl AwayLength {
    pub(crate) fn duration(&self) -> Duration {
        let number = self.number as i64;
        match self.unit {
            AwayLengthUnit::Day => Duration::days(number),
            AwayLengthUnit::Week => Duration::weeks(number),
            AwayLengthUnit::Month => Duration::weeks(4 * number),
        }
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum AwayLengthUnit {
    #[value(alias("days"))]
    Day,

    #[value(alias("weeks"))]
    Week,

    #[value(alias("months"))]
    Month,
}

#[derive(ClapArgs, Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConfigCmd {
    #[command(subcommand)]
    pub(crate) config: Option<Config>,
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) enum Config {
    /// Interact with home profile config
    Home(ProfileConfig),

    /// Interact with work profile config
    Work(ProfileConfig),

    /// Interact with work hours
    WorkHours(WorkHoursConfig),
}

#[derive(ClapArgs, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) struct ProfileConfig {
    /// The setting in the profile to query
    #[clap(name = "setting")]
    pub(crate) key: Option<ConfigKey>,

    /// If present, set the specified setting to this value, otherwise print it
    #[clap(name = "value")]
    pub(crate) value: Option<String>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) enum ConfigKey {
    Browser,
    BackgroundDir,
}

#[derive(ClapArgs, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) struct WorkHoursConfig {
    pub(crate) transition: Option<WorkHoursTransition>,
    pub(crate) time: Option<String>,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) enum WorkHoursTransition {
    ClockOn,
    ClockOff,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn auto() {
        assert_eq!(None, Args::parse_from(["commute"]).command());
        assert_eq!(
            &Command::Auto,
            Args::parse_from(["commute", "auto"]).command().unwrap()
        );
    }

    #[test]
    fn home() {
        assert_eq!(
            &Command::Home,
            Args::parse_from(["commute", "home"]).command().unwrap()
        );
    }

    #[test]
    fn work() {
        assert_eq!(
            &Command::Work,
            Args::parse_from(["commute", "work"]).command().unwrap()
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
