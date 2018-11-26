use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn write(file_name: &PathBuf, content: &[u8]) -> Result<(), String> {
    let mut file = File::create(file_name).map_err(|e| format!("{}", e))?;
    file.write_all(content).map_err(|e| format!("{}", e))
}

pub fn write_enumerated(path: &str, content: &[String]) -> Result<(), String> {
    let p = PathBuf::from(path);
    create_dir_all(&p).map_err(|e| format!("{}", e))?;
    for (i, c) in content.into_iter().enumerate() {
        let mut file_name = p.clone();
        file_name.push(format!("{}.json", i));
        write(&file_name, c.as_bytes())?;
    }
    Ok(())
}