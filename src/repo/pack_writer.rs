use anyhow::Result;
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

pub struct PackWriter {
    pack_file_path: PathBuf,
    pub chunks: Vec<Vec<u8>>,
}

impl PackWriter {
    pub fn new(repo_path: &PathBuf) -> Result<Self> {
        let path_base = repo_path.join("packs/");
        fs::create_dir_all(&path_base)?;

        let dir_count = std::fs::read_dir(&path_base)?.count();
        let filename = format!("{:06}.pack", dir_count);

        let chunks = Vec::new();

        Ok(Self {
            pack_file_path: path_base.join(filename),
            chunks: chunks,
        })
    }

    pub fn append(&mut self, chunk: Vec<u8>) {
        self.chunks.push(chunk);
    }

    pub fn flush(&self, data: &Vec<Vec<u8>>) -> Result<()> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.pack_file_path)?;

        let mut writer = BufWriter::new(file);
        for chunk in data {
            writer.write_all(chunk)?;
        }

        writer.flush()?;

        Ok(())
    }
}
