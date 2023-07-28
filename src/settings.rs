use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

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
    pub(crate) override_day: Option<usize>,
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
        let settings_path = Path::new(&SETTINGS_FILE[..]);
        let settings_dir = settings_path.parent().unwrap();
        fs::create_dir_all(settings_dir)?;

        let mut settings_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(settings_path)?;
        Ok(write!(settings_file, "{}", serde_json::to_string(self)?)?)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work: Profile {
                browser: "/usr/bin/firefox".into(),
                background_dir: "/home/kcza/Pictures/wallpapers/play".into(),
            },
            play: Profile {
                browser: "/snap/bin/brave".into(),
                background_dir: "/home/kcza/Pictures/wallpapers/work".into(),
            },
            override_day: None,
            dirty: false,
        }
    }
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Profile {
    pub(crate) browser: String,
    pub(crate) background_dir: String,
}
