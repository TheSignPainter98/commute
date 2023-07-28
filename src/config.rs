use serde::{Deserialize as Deserialise, Serialize as Serialise};
use std::{error::Error, fs, path::PathBuf};

pub(crate) struct Settings {
    work: Profile,
    play: Profile,
    override_day: Option<usize>,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        println!("hfjdksl");
        let src = fs::read_to_string()?;
        println!("hfjdksl");
        Ok(serde_json::from_str(&src)?)
    }

    pub(crate) fn save(&self) -> Result<(), Box<dyn Error>> {
        todo!();
    }
}

#[derive(Serialise, Deserialise)]
pub(crate) struct Profile {
    browser: String,
    background_dir: PathBuf,
}
