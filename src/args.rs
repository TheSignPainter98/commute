use chrono::Duration;
use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use kinded::Kinded;

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

#[derive(Subcommand, Kinded, Debug, PartialEq, Eq, Default)]
#[warn(missing_docs)]
pub(crate) enum Command {
    /// Guess the place to commute to, can be overridden by calling home or work.
    #[default]
    Auto,

    /// Revert overrides and guess where to commute to the normal place
    Norm,

    /// Set home presets
    Home {
        #[clap(flatten)]
        input_duration: InputDuration,
    },

    /// Set work presets
    Work {
        #[clap(flatten)]
        input_duration: InputDuration,
    },

    /// Change configuration
    Config(ConfigCmd),
}

#[cfg(test)]
impl Command {
    fn input_duration(&self) -> Option<&InputDuration> {
        match self {
            Self::Work { input_duration } | Self::Home { input_duration } => Some(input_duration),
            _ => None,
        }
    }

    fn config(&self) -> Option<&ConfigCmd> {
        match self {
            Self::Config(cfg) => Some(cfg),
            _ => None,
        }
    }
}

#[derive(ClapArgs, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) struct InputDuration {
    /// The duration of the stay
    #[arg(value_name = "duration", requires = "unit")]
    number: Option<u32>,

    /// The units of the duration of the stay
    #[arg(value_name = "units")]
    unit: Option<InputDurationUnit>,
}

impl InputDuration {
    pub(crate) fn duration(&self) -> Duration {
        match self.number {
            Some(number) => {
                let number = number as i64;
                use InputDurationUnit::*;
                match self.unit.expect("internal error: number with no unit") {
                    Minutes => Duration::minutes(number),
                    Hours => Duration::hours(number),
                    Days => Duration::days(number),
                    Weeks => Duration::weeks(number),
                    Months => Duration::days(30 * number), // Approximate
                    Years => Duration::days(365 * number), // Approximate
                }
            }
            None => Self::default().duration(),
        }
    }
}

impl Default for InputDuration {
    fn default() -> Self {
        Self {
            number: Some(10),
            unit: Some(InputDurationUnit::Hours),
        }
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum InputDurationUnit {
    #[value(alias = "minute")]
    Minutes,

    #[value(alias = "hour")]
    Hours,

    #[value(alias = "day")]
    Days,

    #[value(alias = "week")]
    Weeks,

    #[value(alias = "month")]
    Months,

    #[value(alias = "year")]
    Years,
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

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
#[warn(missing_docs)]
pub(crate) enum ConfigKey {
    Browser,
    BackgroundDir,
    GtkTheme,
    IconTheme,
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
    fn default() {
        assert_eq!(CommandKind::Auto, Command::default().kind())
    }

    #[test]
    fn auto() {
        assert_eq!(None, Args::parse_from(["commute"]).command());
        assert_eq!(
            CommandKind::Auto,
            Args::parse_from(["commute", "auto"])
                .command()
                .expect("expected command")
                .kind()
        );
    }

    #[test]
    fn norm() {
        assert_eq!(
            CommandKind::Norm,
            Args::parse_from(["commute", "norm"])
                .command()
                .expect("expected command")
                .kind()
        );
    }

    #[test]
    fn home() {
        test_profile(CommandKind::Home, "home");
    }

    #[test]
    fn work() {
        test_profile(CommandKind::Work, "work");
    }

    fn test_profile(command_kind: CommandKind, command_name: &str) {
        assert_eq!(
            command_kind,
            Args::parse_from(["commute", command_name])
                .command()
                .expect("expected command")
                .kind(),
        );
        assert!(Args::try_parse_from(["commute", command_name, "12"])
            .err()
            .expect("expected error")
            .to_string()
            .contains("<units>"));

        let units = [
            ("minute", InputDurationUnit::Minutes),
            ("minutes", InputDurationUnit::Minutes),
            ("hour", InputDurationUnit::Hours),
            ("hours", InputDurationUnit::Hours),
            ("day", InputDurationUnit::Days),
            ("days", InputDurationUnit::Days),
            ("week", InputDurationUnit::Weeks),
            ("weeks", InputDurationUnit::Weeks),
            ("month", InputDurationUnit::Months),
            ("months", InputDurationUnit::Months),
            ("year", InputDurationUnit::Years),
            ("years", InputDurationUnit::Years),
        ];
        for (raw, unit) in units {
            assert_eq!(
                &InputDuration {
                    number: Some(10),
                    unit: Some(unit)
                },
                Args::parse_from(["commute", command_name, "10", raw])
                    .command()
                    .expect("expected defined command")
                    .input_duration()
                    .expect("test error: expected input duration")
            );
        }
    }

    #[test]
    fn config() {
        assert_eq!(
            CommandKind::Config,
            Args::parse_from(["commute", "config"])
                .command()
                .expect("expected command")
                .kind()
        );
        assert_eq!(
            None,
            Args::parse_from(["commute", "config"])
                .command()
                .expect("expected command")
                .config()
                .expect("expected config")
                .config
        );
        assert_eq!(
            Some(Config::Home(ProfileConfig {
                key: None,
                value: None
            })),
            Args::parse_from(["commute", "config", "home"])
                .command()
                .expect("expected command")
                .config()
                .expect("expected config")
                .config
        );
        assert_eq!(
            Some(Config::Work(ProfileConfig {
                key: None,
                value: None
            })),
            Args::parse_from(["commute", "config", "work"])
                .command()
                .expect("expected command")
                .config()
                .expect("expected config")
                .config
        );

        let keys = [
            (ConfigKey::Browser, "browser"),
            (ConfigKey::BackgroundDir, "background-dir"),
            (ConfigKey::GtkTheme, "gtk-theme"),
            (ConfigKey::IconTheme, "icon-theme"),
        ];
        for (key, raw) in keys {
            assert_eq!(
                Some(Config::Home(ProfileConfig {
                    key: Some(key),
                    value: None
                })),
                Args::parse_from(["commute", "config", "home", raw])
                    .command()
                    .expect("expected command")
                    .config()
                    .expect("expected config")
                    .config
            );
            assert_eq!(
                Some(Config::Home(ProfileConfig {
                    key: Some(key),
                    value: Some("foo".into()),
                })),
                Args::parse_from(["commute", "config", "home", raw, "foo"])
                    .command()
                    .expect("expected command")
                    .config()
                    .expect("expected config")
                    .config
            );
        }

        assert!(matches!(
            Args::parse_from(["commute", "config", "work-hours"])
                .command()
                .expect("expected command")
                .config()
                .expect("expected config command")
                .config
                .as_ref()
                .expect("expected config"),
            Config::WorkHours(WorkHoursConfig {
                transition: None,
                time: None
            }),
        ));
        for (raw, transition) in [
            ("clock-on", WorkHoursTransition::ClockOn),
            ("clock-off", WorkHoursTransition::ClockOff),
        ] {
            let Config::WorkHours(WorkHoursConfig { transition: found_transition, time }) =
                Args::parse_from(["commute", "config", "work-hours", raw])
                    .command()
                    .expect("expected command")
                    .config()
                    .expect("expected config command")
                    .config
                    .clone()
                    .expect("expected config") else {
                        panic!("expected work hours config")
                    };
            assert_eq!(Some(transition), found_transition);
            assert_eq!(time, None);

            let Config::WorkHours(WorkHoursConfig { transition: found_transition, time }) =
                Args::parse_from(["commute", "config", "work-hours", raw, "12:34"])
                    .command()
                    .expect("expected command")
                    .config()
                    .expect("expected config command")
                    .config
                    .clone()
                    .expect("expected config") else {
                        panic!("expected work hours config")
                    };
            assert_eq!(Some(transition), found_transition);
            assert_eq!(Some("12:34"), time.as_deref());
        }
    }
}
