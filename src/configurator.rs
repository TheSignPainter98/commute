use crate::{
    args::{ConfigKey, ProfileType},
    settings::{Profile, Settings},
};

#[derive(Debug)]
pub(crate) struct Configurator<'a> {
    settings: &'a mut Settings,
}

impl<'a> Configurator<'a> {
    pub(crate) fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }

    pub(crate) fn get(&self, profile_type: &ProfileType, key: &ConfigKey) -> &str {
        let profile = self.profile(profile_type);
        match key {
            ConfigKey::Browser => profile.browser(),
            ConfigKey::BackgroundDir => profile.background_dir(),
        }
    }

    pub(crate) fn set(&mut self, profile_type: &ProfileType, key: &ConfigKey, value: &str) {
        let value = value.to_owned();
        let profile = self.profile_mut(profile_type);
        match key {
            ConfigKey::Browser => profile.set_browser(value),
            ConfigKey::BackgroundDir => profile.set_background_dir(value),
        }
    }

    pub(crate) fn profile(&self, profile_type: &ProfileType) -> &Profile {
        match profile_type {
            ProfileType::Home => self.settings.home(),
            ProfileType::Work => self.settings.work(),
        }
    }

    pub(crate) fn profile_mut(&mut self, profile_type: &ProfileType) -> &mut Profile {
        match profile_type {
            ProfileType::Home => self.settings.home_mut(),
            ProfileType::Work => self.settings.work_mut(),
        }
    }

    pub(crate) fn settings(&self) -> &Settings {
        &self.settings
    }
}
