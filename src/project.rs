use crate::frame::Frame;
use crate::input;
use prettytable::{format, Table};
use question::{Answer, Question};
use std::fmt::Display;
use std::path::PathBuf;
use std::fs::File;
use unwrap_infallible::UnwrapInfallible;

pub struct Project {
    frames: Vec<Frame>,
    // TODO: Filter filter
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        table.add_row(prettytable::row!["Name", "Duration", "Path"]);

        for frame in &self.frames {
            table.add_row(prettytable::row![
                frame.name,
                frame.duration,
                frame.path.to_string_lossy()
            ]);
        }

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

        write!(f, "{}", table)
    }
}

type MenuEntry = (&'static str, fn(&mut Project));

pub struct Menu {
    menu: Vec<MenuEntry>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            menu: vec![
                ("Add Frame", Project::add_frame),
                ("Remove Frame", Project::remove_frame),
                ("Move Frame", Project::move_frame),
                ("Change Frame Duration", Project::change_frame_duration),
                (
                    "Change Duration of All Frames",
                    Project::change_duration_of_all_frames,
                ),
                ("Display Frames", |project| println!("{}", project)),
                ("Play", Project::play),
                ("Save Project", Project::save),
            ],
        }
    }

    pub fn len(&self) -> usize {
        self.menu.len()
    }

    pub fn get(&self, index: usize) -> Option<&MenuEntry> {
        self.menu.get(index)
    }
}

impl Display for Menu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, item) in self.menu.iter().enumerate() {
            writeln!(f, " [{}] {}", index + 1, item.0)?;
        }

        Ok(())
    }
}

impl Project {
    pub fn new() -> Project {
        Project { frames: Vec::new() }
    }

    pub fn from(file: &mut File) -> Result<Project, Box<dyn std::error::Error>> {
        let frames: Vec<Frame> = serde_json::from_reader(file)?;
        Ok(Project { frames })
    }

    pub fn add_frame(&mut self) {
        self.frames.push(Frame::from_stdin());
    }

    fn get_frame_index(&self) -> Option<usize> {
        let name: String = input::input().unwrap_infallible();

        let index = self.frames.iter().position(|f| f.name == name);

        if let Some(index) = index {
            Some(index)
        } else {
            println!("Frame not found.");
            None
        }
    }

    pub fn remove_frame(&mut self) {
        println!("Enter the name of the frame to remove: ");
        let Some(index) = self.get_frame_index() else { return; };

        self.frames.remove(index);
    }

    pub fn move_frame(&mut self) {
        println!("Enter the name of the frame to move: ");
        let Some(frame_index) = self.get_frame_index() else { return; };

        println!("Enter the new index: ");
        let new_index =
            input::input_valid_and_also(|idx: &usize| *idx <= self.frames.len() && *idx > 0) - 1;

        let frame = self.frames.remove(frame_index);
        self.frames.insert(new_index, frame);
    }

    pub fn change_frame_duration(&mut self) {
        println!("Enter the name of the frame to change the duration: ");
        let Some(frame_index) = self.get_frame_index() else { return; };

        println!("Enter the new duration: ");
        let duration = input::input_valid();

        self.frames[frame_index].duration = duration;
    }

    pub fn change_duration_of_all_frames(&mut self) {
        println!("Enter the new duration for the frames: ");
        let duration = input::input_valid();

        self.frames
            .iter_mut()
            .for_each(|frame| frame.duration = duration);
    }

    pub fn save(&mut self) {
        println!("Enter the path of the file to save into: ");
        let path: PathBuf = input::input().unwrap_infallible();

        if path.exists() {
            println!("The given path exists.");
            let answer = Question::new("Do you want to overwrite?").confirm();

            if answer == Answer::NO {
                println!("Aborting");
                return;
            }
        }

        match File::create(path) {
            Ok(file) => match serde_json::to_writer(file, &self.frames) {
                Ok(_) => println!("Saved"),
                Err(e) => println!("Error writing to the file: {}", e),
            },
            Err(e) => println!("Error opening the file: {}", e),
        }
    }

    pub fn play(&mut self) {
        const WINDOW_NAME: &str = "Gif Maker";

        if self.frames.is_empty() {
            println!("No frames to display");
            return;
        }

        opencv::highgui::named_window(WINDOW_NAME, opencv::highgui::WINDOW_AUTOSIZE)
            .expect("Failed to create window");

        loop {
            for frame in &self.frames {
                let img = match opencv::imgcodecs::imread(
                    &frame.path.to_string_lossy(),
                    opencv::imgcodecs::IMREAD_COLOR,
                ) {
                    Ok(img) => img,
                    Err(e) => {
                        println!("Failed to open {}: {}", frame.path.to_string_lossy(), e);
                        continue;
                    }
                };

                let _ = opencv::highgui::imshow(WINDOW_NAME, &img);

                let key_was_pressed =
                    opencv::highgui::wait_key(frame.duration.into()).unwrap_or_default() != -1;

                if key_was_pressed {
                    opencv::highgui::destroy_window(WINDOW_NAME).expect("Destroying window failed");
                    return;
                }
            }
        }
    }
}
