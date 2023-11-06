use std::{
    fs::{File, OpenOptions},
    io::{Error, Seek, Write},
};

pub struct Device {
    file: File,
}

impl Device {
    pub fn new(path: &str) -> Device {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        Device { file }
    }

    pub fn write(&self, data: &[u8]) -> Result<(), Error> {
        let mut file = self.file.try_clone()?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.flush()?;
        file.write_all(data)?;
        Ok(())
    }
}
