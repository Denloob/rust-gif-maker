mod frame;
mod input;
mod project;

use project::{Menu, Project};
use question::{Answer, Question};
use std::{fs::File, path::PathBuf};
use unwrap_infallible::UnwrapInfallible;

fn on_project_load_error(e: Box<dyn std::error::Error>) -> Project {
    println!("Error loading project: {}", e);
    println!("Creating a new project instead.");
    Project::new()
}

fn main() {
    println!("Welcome!");
    let should_load_project = Question::new("Do you want to load an existing project?").confirm();

    let mut project = if should_load_project == Answer::YES {
        println!("Please enter a path to the project:");
        let path: PathBuf = input::input().unwrap_infallible();

        File::open(path)
            .map_or_else(
                |e| Ok(on_project_load_error(Box::new(e))),
                |mut file| Project::from(&mut file),
            )
            .unwrap_or_else(on_project_load_error)
    } else {
        Project::new()
    };

    let menu = Menu::new();

    loop {
        println!("\nWhat would you like to do?\n [0] Exit\n{}", menu);

        let func_index = input::input_valid_and_also(|idx: &usize| *idx <= menu.len());
        if func_index == 0 {
            return;
        }

        let (_, func) = menu
            .get(func_index - 1)
            .expect("We already tested for index being valid.");

        func(&mut project);
    }
}
