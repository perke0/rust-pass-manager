use std::fs::OpenOptions;
use std::io::{self, Read, Write};

#[allow(dead_code)]
pub fn read_data(path: &str) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[allow(dead_code)]
pub fn write_data(file_path: &str, data: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    file.write_all(data.as_bytes())?;

    Ok(())
}

#[allow(dead_code)]
pub fn create_file(file_path: &str) -> io::Result<()> {
    let _file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    Ok(())
}
