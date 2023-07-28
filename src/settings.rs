use serde::{Deserialize as Deserialise, Serialize as Serialise};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

const SETTINGS_FILE: &str = "~/.config/share/levo/";

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Settings {
    work: Profile,
    play: Profile,
    override_day: Option<usize>,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        if let Ok(src) = fs::read_to_string(SETTINGS_FILE) {
            Ok(serde_json::from_str(&src)?)
        } else {
            Ok(Default::default())
        }
    }

    pub(crate) fn save(&self) -> Result<(), Box<dyn Error>> {
        let settings_path = Path::new(SETTINGS_FILE);
        let settings_dir = settings_path.parent().unwrap();
        fs::create_dir_all(settings_dir)?;

        let mut settings_file = OpenOptions::new().create(true).open(settings_path)?;
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
        }
    }
}

#[derive(Debug, Serialise, Deserialise)]
pub(crate) struct Profile {
    browser: String,
    background_dir: PathBuf,
}
