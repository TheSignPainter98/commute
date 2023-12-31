use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::Context;
use chrono::{offset::Utc, serde::ts_seconds, DateTime, Duration, Local, NaiveTime};
use directories::ProjectDirs;
use lazy_static::lazy_static;
use serde::{Deserialize as Deserialise, Serialize as Serialise};

use crate::result::Result;

lazy_static! {
    static ref SETTINGS_PATH: PathBuf = {
        ProjectDirs::from("net", "kcza", env!("CARGO_PKG_NAME"))
            .unwrap()
            .data_local_dir()
            .join("settings.yml")
    };
}

#[derive(Debug, Serialise, Deserialise)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Settings {
    work: Profile,
    home: Profile,
    work_hours: WorkHours,
    r#override: Option<Override>,

    #[serde(skip)]
    dirty: bool,
}

impl Settings {
    pub(crate) fn new() -> Result<Self> {
        if let Ok(src) = fs::read_to_string(&*SETTINGS_PATH) {
            Ok(serde_yaml::from_str(&src)?)
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
        Ok(write!(settings_file, "{}", serde_yaml::to_string(self)?)?)
    }

    fn is_dirty(&self) -> bool {
        self.dirty || self.work.dirty() || self.home.dirty() || self.work_hours.dirty()
    }

    pub(crate) fn work(&self) -> &Profile {
        &self.work
    }

    pub(crate) fn work_mut(&mut self) -> &mut Profile {
        &mut self.work
    }

    pub(crate) fn home(&self) -> &Profile {
        &self.home
    }

    pub(crate) fn home_mut(&mut self) -> &mut Profile {
        &mut self.home
    }

    pub(crate) fn work_hours(&self) -> &WorkHours {
        &self.work_hours
    }

    pub(crate) fn work_hours_mut(&mut self) -> &mut WorkHours {
        &mut self.work_hours
    }

    pub(crate) fn r#override(&self) -> Option<&Override> {
        self.r#override.as_ref()
    }

    pub(crate) fn set_override(&mut self, r#override: Override) {
        self.dirty = true;
        self.r#override = Some(r#override);
    }

    pub(crate) fn reset_override(&mut self) {
        self.dirty = true;
        self.r#override = None;
    }
}

impl Default for Settings {
    fn default() -> Self {
        lazy_static! {
            static ref DEFAULT_WORK_START: NaiveTime = NaiveTime::from_hms_opt(6, 0, 0).unwrap();
            static ref DEFAULT_WORK_END: NaiveTime = NaiveTime::from_hms_opt(18, 30, 00).unwrap();
        };

        Self {
            work: Profile {
                browser: None,
                background_dir: None,
                dirty: false,
                theme: Theme {
                    gtk: None,
                    icons: None,
                    dirty: false,
                },
            },
            home: Profile {
                browser: None,
                background_dir: None,
                dirty: false,
                theme: Theme {
                    gtk: None,
                    icons: None,
                    dirty: false,
                },
            },
            work_hours: WorkHours {
                clock_on: *DEFAULT_WORK_START,
                clock_off: *DEFAULT_WORK_END,
                dirty: false,
            },
            r#override: None,
            dirty: false,
        }
    }
}

#[derive(Debug, Serialise, Deserialise)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Profile {
    browser: Option<String>,
    background_dir: Option<String>,

    #[serde(flatten)]
    theme: Theme,

    #[serde(skip)]
    dirty: bool,
}

impl Profile {
    pub(crate) fn browser(&self) -> Option<&str> {
        self.browser.as_deref()
    }

    pub(crate) fn set_browser(&mut self, browser: Option<String>) {
        self.dirty = true;
        self.browser = browser;
    }

    pub(crate) fn background_dir(&self) -> Option<&str> {
        self.background_dir.as_deref()
    }

    pub(crate) fn set_background_dir(&mut self, background_dir: Option<String>) {
        self.dirty = true;
        self.background_dir = background_dir;
    }

    pub(crate) fn theme(&self) -> &Theme {
        &self.theme
    }

    pub(crate) fn theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    pub(crate) fn dirty(&self) -> bool {
        self.dirty || self.theme.dirty()
    }
}

#[derive(Copy, Clone, Debug, Serialise, Deserialise, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[warn(missing_docs)]
pub(crate) enum ProfileType {
    Work,
    Home,
}

#[derive(Clone, Debug, Serialise, Deserialise, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[warn(missing_docs)]
pub(crate) struct Theme {
    #[serde(rename = "gtk-theme")]
    gtk: Option<String>,
    #[serde(rename = "icon-theme")]
    icons: Option<String>,

    #[serde(skip)]
    dirty: bool,
}

impl Theme {
    pub(crate) fn gtk(&self) -> Option<&str> {
        self.gtk.as_deref()
    }

    pub(crate) fn set_gtk(&mut self, gtk: Option<String>) {
        self.dirty = true;
        self.gtk = gtk;
    }

    pub(crate) fn icons(&self) -> Option<&str> {
        self.icons.as_deref()
    }

    pub(crate) fn set_icons(&mut self, icons: Option<String>) {
        self.dirty = true;
        self.icons = icons;
    }

    pub(crate) fn dirty(&self) -> bool {
        self.dirty
    }
}

#[derive(Debug, Serialise, Deserialise)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct WorkHours {
    clock_on: NaiveTime,
    clock_off: NaiveTime,

    #[serde(skip)]
    dirty: bool,
}

impl WorkHours {
    pub(crate) fn start(&self) -> &NaiveTime {
        &self.clock_on
    }

    pub(crate) fn set_start(&mut self, start: NaiveTime) {
        self.dirty = true;
        self.clock_on = start;
    }

    pub(crate) fn end(&self) -> &NaiveTime {
        &self.clock_off
    }

    pub(crate) fn set_end(&mut self, end: NaiveTime) {
        self.dirty = true;
        self.clock_off = end;
    }

    pub(crate) fn dirty(&self) -> bool {
        self.dirty
    }
}

#[derive(Debug, Serialise, Deserialise)]
#[serde(rename_all = "kebab-case")]
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
