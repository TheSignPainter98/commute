use chrono::NaiveTime;

use crate::{
    args::{ConfigKey, WorkHoursTransition},
    settings::{Profile, ProfileType, Settings, WorkHours},
};

#[derive(Debug)]
pub(crate) struct Configurator<'a> {
    settings: &'a mut Settings,
}

impl<'a> Configurator<'a> {
    pub(crate) fn new(settings: &'a mut Settings) -> Self {
        Self { settings }
    }

    pub(crate) fn get(&self, profile_type: ProfileType, key: &ConfigKey) -> &str {
        let profile = self.profile(profile_type);
        match key {
            ConfigKey::Browser => profile.browser(),
            ConfigKey::BackgroundDir => profile.background_dir(),
        }
    }

    pub(crate) fn set(&mut self, profile_type: ProfileType, key: &ConfigKey, value: &str) {
        let value = value.to_owned();
        let profile = self.profile_mut(profile_type);
        match key {
            ConfigKey::Browser => profile.set_browser(value),
            ConfigKey::BackgroundDir => profile.set_background_dir(value),
        }
    }

    pub(crate) fn profile(&self, profile_type: ProfileType) -> &Profile {
        match profile_type {
            ProfileType::Home => self.settings.home(),
            ProfileType::Work => self.settings.work(),
        }
    }

    pub(crate) fn profile_mut(&mut self, profile_type: ProfileType) -> &mut Profile {
        match profile_type {
            ProfileType::Home => self.settings.home_mut(),
            ProfileType::Work => self.settings.work_mut(),
        }
    }

    pub(crate) fn clocking_times(&self) -> &WorkHours {
        self.settings.work_hours()
    }

    pub(crate) fn clocking_time(&self, transition: WorkHoursTransition) -> &NaiveTime {
        match transition {
            WorkHoursTransition::ClockOn => self.settings.work_hours().start(),
            WorkHoursTransition::ClockOff => self.settings.work_hours().end(),
        }
    }

    pub(crate) fn set_clocking_time(&mut self, transition: WorkHoursTransition, time: NaiveTime) {
        use WorkHoursTransition::*;
        match transition {
            ClockOn => self.settings.work_hours_mut().set_start(time),
            ClockOff => self.settings.work_hours_mut().set_end(time),
        }
    }

    pub(crate) fn settings(&self) -> &Settings {
        &self.settings
    }
}
