use std::{
    fs::File,
    io::{BufReader, Write},
    str,
};

use crate::model::files::FileData;

pub trait FileManager<T> {
    fn read(path: &str) -> T;
    fn write(path: &str, content: String);
}

pub struct SongsFileManager;

impl FileManager<FileData> for SongsFileManager {
    fn read(path: &str) -> FileData {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    fn write(path: &str, content: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}
