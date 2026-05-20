use std::fs::{self, File};
use std::io::{self, BufWriter};
use std::path::Path;

use crate::task::Task;

const LISTS_DIR: &str = "lists";

pub fn ensure_lists_dir() -> io::Result<()> {
    fs::create_dir_all(LISTS_DIR)
}

pub fn list_names() -> io::Result<Vec<String>> {
    let mut names: Vec<String> = fs::read_dir(LISTS_DIR)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.strip_suffix(".json").map(|s| s.to_string())
        })
        .collect();
    names.sort();
    Ok(names)
}

pub fn load_list(name: &str) -> io::Result<Vec<Task>> {
    let path = format!("{}/{}.json", LISTS_DIR, name);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e),
    };
    if file.metadata()?.len() == 0 {
        return Ok(vec![]);
    }
    let tasks: Vec<Task> = serde_json::from_reader(file)?;
    Ok(tasks)
}

pub fn save_list(name: &str, tasks: &[Task]) -> io::Result<()> {
    let path = format!("{}/{}.json", LISTS_DIR, name);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, tasks)?;
    Ok(())
}

pub fn delete_list(name: &str) -> io::Result<()> {
    let path = format!("{}/{}.json", LISTS_DIR, name);
    fs::remove_file(path)
}

pub fn migrate_legacy(legacy_path: &str, default_name: &str) -> io::Result<()> {
    if !Path::new(legacy_path).exists() {
        return Ok(());
    }
    let file = File::open(legacy_path)?;
    let tasks: Vec<Task> = if file.metadata()?.len() == 0 {
        vec![]
    } else {
        serde_json::from_reader(file)?
    };
    save_list(default_name, &tasks)?;
    fs::remove_file(legacy_path)?;
    Ok(())
}
