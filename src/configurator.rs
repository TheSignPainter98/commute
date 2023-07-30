use crate::{
    args::{ConfigKey, ProfileType},
    settings::Settings,
};

pub(crate) struct Configurator<'a> {
    settings: &'a mut Settings,
}

impl<'a> Configurator<'a> {
    pub(crate) fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }

    pub(crate) fn get(&self, profile_type: &ProfileType, key: &ConfigKey) -> &str {
        match (profile_type, key) {
            (ProfileType::Work, ConfigKey::Browser) => self.settings.work().browser(),
            (ProfileType::Work, ConfigKey::BackgroundDir) => self.settings.work().background_dir(),
            (ProfileType::Play, ConfigKey::Browser) => self.settings.play().browser(),
            (ProfileType::Play, ConfigKey::BackgroundDir) => self.settings.play().background_dir(),
        }
    }

    pub(crate) fn set(&mut self, profile_type: &ProfileType, key: &ConfigKey, value: &str) {
        let value = value.to_owned();
        match (profile_type, key) {
            (ProfileType::Play, ConfigKey::Browser) => self.settings.play_mut().set_browser(value),
            (ProfileType::Play, ConfigKey::BackgroundDir) => {
                self.settings.play_mut().set_background_dir(value)
            }
            (ProfileType::Work, ConfigKey::Browser) => self.settings.work_mut().set_browser(value),
            (ProfileType::Work, ConfigKey::BackgroundDir) => {
                self.settings.work_mut().set_background_dir(value)
            }
        }
    }
}
