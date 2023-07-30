use std::error::Error;

use crate::{
    args::{ConfigOption},
    settings::Settings,
};

pub(crate) struct Configurator<'a> {
    settings: &'a mut Settings,
}

impl<'a> Configurator<'a> {
    pub(crate) fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }

    pub(crate) fn config(&mut self, config_option: &ConfigOption) -> Result<(), Box<dyn Error>> {
        match config_option {
            ConfigOption::WorkBrowser { browser } => match browser {
                None => println!("{}", self.settings.work().browser()),
                Some(browser) => self.settings.work_mut().set_browser(browser.clone()),
            },
            ConfigOption::PlayBrowser { browser } => match browser {
                None => println!("{}", self.settings.play().browser()),
                Some(browser) => self.settings.play_mut().set_browser(browser.clone()),
            },
            ConfigOption::WorkBackgroundDir { dir } => match dir {
                None => println!("{}", self.settings.work().background_dir()),
                Some(dir) => self.settings.work_mut().set_background_dir(dir.clone()),
            },
            ConfigOption::PlayBackgroundDir { dir } => match dir {
                None => println!("{}", self.settings.play().background_dir()),
                Some(dir) => self.settings.play_mut().set_background_dir(dir.clone()),
            },
        }
        Ok(())
    }
}
