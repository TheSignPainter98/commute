use std::fs;
use std::{ffi::OsStr, process::Command};

use chrono::{Datelike, Local, NaiveTime};
use gio::prelude::SettingsExt;
use lazy_static::lazy_static;
use rand::seq::SliceRandom;

use crate::{
    args::ProfileType,
    result::Result,
    settings::{Override, Profile, Settings},
};

lazy_static! {
    static ref WORK_START: NaiveTime = NaiveTime::from_hms_opt(6, 0, 0).unwrap();
    static ref WORK_END: NaiveTime = NaiveTime::from_hms_opt(18, 30, 00).unwrap();
}

pub(crate) struct ProfileApplicator<'a> {
    settings: &'a Settings,
    profile_type: ProfileType,
}

impl<'a> ProfileApplicator<'a> {
    pub(crate) fn new(settings: &'a Settings, profile_type: ProfileType) -> Self {
        Self {
            settings,
            profile_type,
        }
    }

    pub(crate) fn auto(settings: &'a Settings) -> Self {
        let profile_type = settings
            .r#override()
            .and_then(Override::advise_profile)
            .unwrap_or_else(|| {
                let now = Local::now();
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
        Self::new(settings, profile_type)
    }

    pub(crate) fn apply(&self) -> Result<()> {
        match self.profile_type {
            ProfileType::Work => self.apply_profile(self.settings.work()),
            ProfileType::Play => self.apply_profile(self.settings.play()),
        }
    }

    fn apply_profile(&self, profile: &Profile) -> Result<()> {
        self.set_browser(profile)?;
        self.set_background(profile)?;

        Ok(())
    }

    fn set_browser(&self, profile: &Profile) -> Result<()> {
        let status = Command::new("xdg-settings")
            .arg("set")
            .arg("default-web-browser")
            .arg(profile.browser())
            .status()?;
        if !status.success() {
            Err(crate::error::Error::ChildProcessError {
                name: "xdg-settings".into(),
                reason: status.code().into(),
            })
        } else {
            Ok(())
        }
    }

    fn set_background(&self, profile: &Profile) -> Result<()> {
        let background_settings = gio::Settings::new("org.gnome.desktop.background");
        let current_background_uri = background_settings.string("picture-uri-dark").to_string();

        let bkg_uris = {
            let mut bkg_uris = self.available_backgrounds(profile)?;
            bkg_uris.shuffle(&mut rand::thread_rng());
            bkg_uris
        };
        if let Some(uri) = bkg_uris.iter().find(|u| *u != &current_background_uri) {
            background_settings.set_string("picture-uri", uri.as_ref())?;
            background_settings.set_string("picture-uri-dark", uri.as_ref())?;
            gio::Settings::sync();
        }
        Ok(())
    }

    fn available_backgrounds(&self, profile: &Profile) -> Result<Vec<String>> {
        Ok(fs::read_dir(profile.background_dir())?
            .filter_map(|d| {
                if let Ok(d) = d {
                    let path = d.path();
                    let extension = path.extension().map(OsStr::to_str).flatten();
                    match extension {
                        Some("png") | Some("jpg") | Some("jpeg") => Some(path),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .map(|p| p.to_string_lossy().to_string())
            .collect())
    }
}
