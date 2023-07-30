use std::error::Error;

use chrono::{Datelike, NaiveTime, Utc};
use lazy_static::lazy_static;

use crate::{
    args::Args,
    settings::{Override, Profile, ProfileType, Settings},
};

lazy_static! {
    static ref WORK_START: NaiveTime = NaiveTime::from_hms_opt(6, 0, 0).unwrap();
    static ref WORK_END: NaiveTime = NaiveTime::from_hms_opt(18, 30, 00).unwrap();
}

pub(crate) struct ProfileApplier<'a> {
    args: &'a Args,
    settings: &'a Settings,
    profile_type: ProfileType,
}

impl<'a> ProfileApplier<'a> {
    pub(crate) fn new(args: &'a Args, settings: &'a Settings, profile_type: ProfileType) -> Self {
        Self {
            args,
            settings,
            profile_type,
        }
    }

    pub(crate) fn auto(args: &'a Args, settings: &'a Settings) -> Self {
        let profile_type = settings
            .r#override()
            .map(Override::advise_profile)
            .flatten()
            .unwrap_or_else(|| {
                let now = Utc::now();
                use chrono::Weekday::*;
                match now.weekday() {
                    Sat | Sun => ProfileType::Play,
                    _ => {
                        let time = now.time();
                        if *WORK_START <= time && time <= *WORK_END {
                            ProfileType::Work
                        } else {
                            ProfileType::Play
                        }
                    }
                }
            });
        Self::new(args, settings, profile_type)
    }

    pub(crate) fn apply(&self) -> Result<(), Box<dyn Error>> {
        match self.profile_type {
            ProfileType::Work => self.apply_profile(self.settings.work()),
            ProfileType::Play => self.apply_profile(self.settings.play()),
        }
    }

    fn apply_profile(&self, profile: &Profile) -> Result<(), Box<dyn Error>> {
        todo!("apply profile {:?}", self.profile_type);
    }
}
