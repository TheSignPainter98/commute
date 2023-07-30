use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use chrono::{offset::Utc, serde::ts_seconds, DateTime, Duration};
use lazy_static::lazy_static;
use serde::{Deserialize as Deserialise, Serialize as Serialise};

lazy_static! {
    static ref SETTINGS_FILE: String =
        shellexpand::tilde("~/.local/share/levo/settings.json").to_string();
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Settings {
    pub(crate) work: Profile,
    pub(crate) play: Profile,
    pub(crate) r#override: Option<Override>,

    #[serde(skip)]
    dirty: bool,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        if let Ok(src) = fs::read_to_string(&SETTINGS_FILE[..]) {
            Ok(serde_json::from_str(&src)?)
        } else {
            Ok(Default::default())
        }
    }

    pub(crate) fn save(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_dirty() {
            return Ok(());
        }

        let settings_path = Path::new(&SETTINGS_FILE[..]);
        let settings_dir = settings_path.parent().unwrap();
        fs::create_dir_all(settings_dir)?;

        let mut settings_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(settings_path)?;
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

    pub(crate) fn play(&mut self) -> &Profile {
        &self.play
    }

    pub(crate) fn play_mut(&mut self) -> &mut Profile {
        &mut self.play
    }

    pub(crate) fn r#override(&self) -> Option<&Override> {
        self.r#override.as_ref()
    }

    pub(crate) fn declare_override_day(&mut self, profile_type: ProfileType) {
        let date = Utc::now();
        self.r#override = Some(Override { date, profile_type })
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work: Profile {
                browser: "/usr/bin/firefox".into(),
                background_dir: "/home/kcza/Pictures/wallpapers/play".into(),
                dirty: false,
            },
            play: Profile {
                browser: "/snap/bin/brave".into(),
                background_dir: "/home/kcza/Pictures/wallpapers/work".into(),
                dirty: false,
            },
            r#override: None,
            dirty: false,
        }
    }
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Profile {
    pub(crate) browser: String,
    pub(crate) background_dir: String,

    #[serde(skip)]
    dirty: bool,
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Override {
    profile_type: ProfileType,

    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,
}

impl Override {
    pub(crate) fn advise_profile(&self) -> Option<ProfileType> {
        if !self.is_in_force() {
            return None;
        }
        Some(self.profile_type)
    }

    fn is_in_force(&self) -> bool {
        self.date < Utc::now() - self.duration()
    }

    fn duration(&self) -> Duration {
        Duration::hours(6)
    }
}

#[derive(Copy, Clone, Debug, Serialise, Deserialise)]
pub(crate) enum ProfileType {
    Work,
    Play,
}
