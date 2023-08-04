use std::fs;
use std::{ffi::OsStr, process::Command};

use anyhow::Context;
use chrono::{Datelike, Local};
use gio::prelude::SettingsExt;
use rand::seq::SliceRandom;

use crate::{
    result::Result,
    settings::{Override, Profile, ProfileType, Settings},
};

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
                    Sat | Sun => ProfileType::Home,
                    _ => {
                        let time = &now.time();
                        let work_hours = settings.work_hours();
                        if work_hours.start() <= time && time < work_hours.end() {
                            ProfileType::Work
                        } else {
                            ProfileType::Home
                        }
                    }
                }
            });
        Self::new(settings, profile_type)
    }

    pub(crate) fn apply(&self) -> Result<()> {
        match self.profile_type {
            ProfileType::Work => self.apply_profile(self.settings.work()),
            ProfileType::Home => self.apply_profile(self.settings.home()),
        }
    }

    fn apply_profile(&self, profile: &Profile) -> Result<()> {
        self.set_browser(profile).context("failed to set profile")?;
        self.set_background(profile)
            .context("failed to set browser")?;

        gio::Settings::sync();
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
            let mut bkg_uris = self.available_backgrounds(profile).context(format!(
                "failed to find backgrounds in {}",
                profile.background_dir()
            ))?;
            bkg_uris.shuffle(&mut rand::thread_rng());
            bkg_uris
        };
        if let Some(uri) = bkg_uris.iter().find(|u| *u != &current_background_uri) {
            background_settings
                .set_string("picture-uri", uri.as_ref())
                .context("failed to set picture-uri")?;
            background_settings
                .set_string("picture-uri-dark", uri.as_ref())
                .context("failed to set picture-uri")?;
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
