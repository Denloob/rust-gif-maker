use crate::input;
use std::path::PathBuf;
use unwrap_infallible::UnwrapInfallible;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub name: String,
    pub path: PathBuf,
    pub duration: u32,
}

impl Frame {
    pub fn new(name: String, path: PathBuf, duration: u32) -> Frame {
        if !path.is_file() {
            panic!("Given path doesn't exist.");
        }

        Frame {
            name,
            path,
            duration,
        }
    }

    pub fn from_stdin() -> Frame {
        println!("Please enter the name of the new frame: ");
        let name = input::input().unwrap_infallible();

        println!("Please enter the path of the new frame (must be to a existing file): ");
        let path = input::input_valid_and_also(|path: &PathBuf| path.is_file());

        println!("Please enter the duration of the new frame: ");
        let duration = input::input_valid();

        Self::new(name, path, duration)
    }
}
