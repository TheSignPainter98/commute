use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::Context;
use chrono::{offset::Utc, serde::ts_seconds, DateTime, Duration, Local};
use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::{args::ProfileType, result::Result};

lazy_static! {
    static ref SETTINGS_PATH: PathBuf = {
        ProjectDirs::from("net", "kcza", env!("CARGO_PKG_NAME"))
            .unwrap()
            .data_local_dir()
            .join("settings.json")
    };
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Settings {
    work: Profile,
    play: Profile,
    r#override: Option<Override>,

    #[serde(skip)]
    dirty: bool,
}

impl Settings {
    pub(crate) fn new() -> Result<Self> {
        if let Ok(src) = fs::read_to_string(&*SETTINGS_PATH) {
            Ok(serde_json::from_str(&src)?)
        } else {
            Ok(Default::default())
        }
    }

    pub(crate) fn save(&self) -> Result<()> {
        if !self.is_dirty() {
            return Ok(());
        }

        let settings_dir = SETTINGS_PATH.parent().unwrap();
        fs::create_dir_all(settings_dir).context("failed to create parent directories")?;

        let mut settings_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&*SETTINGS_PATH)
            .context(format!("failed to write to {}", SETTINGS_PATH.display()))?;
        Ok(write!(settings_file, "{}", serde_json::to_string(self)?)?)
    }

    fn is_dirty(&self) -> bool {
        self.dirty || self.work.dirty || self.play.dirty
    }

    pub(crate) fn work(&self) -> &Profile {
        &self.work
    }

    pub(crate) fn work_mut(&mut self) -> &mut Profile {
        &mut self.work
    }

    pub(crate) fn play(&self) -> &Profile {
        &self.play
    }

    pub(crate) fn play_mut(&mut self) -> &mut Profile {
        &mut self.play
    }

    pub(crate) fn r#override(&self) -> Option<&Override> {
        self.r#override.as_ref()
    }

    pub(crate) fn set_override(&mut self, r#override: Override) {
        self.dirty = true;
        self.r#override = Some(r#override);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work: Profile {
                browser: "firefox_firefox.desktop".into(),
                background_dir: "/home/kcza/Pictures/wallpapers/work".into(),
                dirty: false,
            },
            play: Profile {
                browser: "brave_brave.desktop".into(),
                background_dir: "/home/kcza/Pictures/wallpapers/play".into(),
                dirty: false,
            },
            r#override: None,
            dirty: false,
        }
    }
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Profile {
    browser: String,
    background_dir: String,

    #[serde(skip)]
    dirty: bool,
}

impl Profile {
    pub(crate) fn browser(&self) -> &str {
        &self.browser
    }

    pub(crate) fn set_browser(&mut self, browser: String) {
        self.dirty = true;
        self.browser = browser;
    }

    pub(crate) fn background_dir(&self) -> &str {
        &self.background_dir
    }

    pub(crate) fn set_background_dir(&mut self, background_dir: String) {
        self.dirty = true;
        self.background_dir = background_dir;
    }
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Override {
    profile_type: ProfileType,

    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}

impl Override {
    pub(crate) fn new(profile_type: ProfileType, duration: Duration) -> Self {
        let date = (Local::now() + duration).into();
        Self { date, profile_type }
    }

    pub(crate) fn advise_profile(&self) -> Option<ProfileType> {
        if !self.is_in_force() {
            return None;
        }
        Some(self.profile_type)
    }

    fn is_in_force(&self) -> bool {
        self.date >= Local::now()
    }
}
