use std::fs::{create_dir_all, write};

use serde_json::from_str;

use crate::task::{Task, Tasks};

const SAVE_DIR: &str = "./data/save";

pub fn save_tasks(tasks: &Tasks) -> Result<(), anyhow::Error> {
    create_dir_all(SAVE_DIR)?;
    let save_path = format!("{}/{}.json", SAVE_DIR, "tuidui_save");
    let serialized = serde_json::to_string_pretty(&tasks)?;
    write(save_path, serialized)?;
    Ok(())
}

pub fn load_tasks() -> Result<Tasks, anyhow::Error> {
    let mut tasks = Tasks::new();

    for file in std::fs::read_dir(SAVE_DIR)? {
        let path = file?.path();
        let contents = std::fs::read_to_string(path)?;
        let mut deserialized: Tasks = from_str(&contents)?;
        tasks.tasks.append(&mut deserialized.tasks);
    }

    Ok(tasks)
}
