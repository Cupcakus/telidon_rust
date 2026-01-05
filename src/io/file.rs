use std::fs::File;
use std::io::Read;
use std::time::Duration;

pub struct TelidonFile {
    file: File,
    baud_rate: u32,
    current_time: u64
}

impl TelidonFile {
    pub fn new(path: &str, baud_rate: u32) -> Self {
        Self { file: File::open(path).unwrap(), baud_rate, current_time: 0 }
    }

    pub fn tick(&mut self, dt: Duration) -> Option<[u8]> {
        let mut buffer = [0; 1];
        self.file.read(&mut buffer).unwrap();
        self.current_time += dt.as_nanos() as u64;
        match self.file.read(&mut buffer) {
            1 => { Some(buffer[0]) }
            _ => { None }
        }
    }
}