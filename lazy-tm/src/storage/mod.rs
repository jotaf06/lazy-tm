use std::fs::File;
use std::io::{self, BufWriter};

use crate::task::Task;

pub fn load(path: &str) -> io::Result<Vec<Task>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => return Ok(vec![]),
            _ => return Err(err),
        },
    };

    if file.metadata()?.len() == 0 {
        return Ok(vec![]);
    }

    let tasks: Vec<Task> = serde_json::from_reader(file)?;

    Ok(tasks)
}

pub fn save(path: &str, tasks: &[Task]) -> io::Result<()> {
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, tasks)?;

    Ok(())
}
