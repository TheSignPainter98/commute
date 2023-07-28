use clap::{Parser, Subcommand};

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
    #[command(subcommand)]
    Config(Config),
}

#[derive(Subcommand, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Config {
    WorkBrowser { browser: Option<String> },
    PlayBrowser { browser: Option<String> },
    WorkBackgroundDir { dir: Option<String> },
    PlayBackgroundDir { dir: Option<String> },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn auto() {
        assert_eq!(None, Args::parse_from(&["levo"]).command());
        assert_eq!(
            &Command::Auto,
            Args::parse_from(&["levo", "auto"]).command().unwrap()
        );
    }

    #[test]
    fn relax() {
        assert_eq!(
            &Command::Play,
            Args::parse_from(&["levo", "relax"]).command().unwrap()
        );
    }

    #[test]
    fn work() {
        assert_eq!(
            &Command::Work,
            Args::parse_from(&["levo", "work"]).command().unwrap()
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
