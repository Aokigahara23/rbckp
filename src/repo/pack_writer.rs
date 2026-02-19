use anyhow::Result;
use std::{
    fs::{self, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

pub struct PackWriter {
    pack_file_path: Path,
    max_pack_size_bytes: u64,
    pub chunks: Vec<Vec<u8>>,
}

impl PackWriter {
    pub fn new(repo_path: &Path, max_pack_size_bytes: u64) -> Result<Self> {
        let path_base = repo_path.join("packs/");
        fs::create_dir_all(&path_base)?;

        let dir_count = std::fs::read_dir(&path_base)?.count();
        let filename = format!("{:06}.pack", dir_count);

        let chunks = Vec::new();

        Ok(Self {
            pack_file_path: path_base.join(filename),
            max_pack_size_bytes: max_pack_size_bytes,
            chunks: chunks,
        })
    }

    pub fn find_available(repo_path: &PathBuf, max_pack_size_bytes: u64) -> Result<Self> {
        let path_base = repo_path.join("packs/");
        if !path_base.exists() {
            return Err(anyhow::anyhow!("packs folder does not exist."));
        }

        let dir = std::fs::read_dir(&path_base)?;
        for file_entry_res in dir {
            let file_entry = file_entry_res?;
            let file_meta = std::fs::metadata(&file_entry.path())?;
            if file_meta.len() < max_pack_size_bytes {
                return Ok(Self {
                    pack_file_path: file_entry.path(),
                    max_pack_size_bytes: max_pack_size_bytes,
                    chunks: Vec::new(),
                });
            }
        }

        Err(anyhow::anyhow!("Could not find free packs"))
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

    pub fn is_full(&self) -> Result<bool> {
        let pack_file_meta = std::fs::metadata(&self.pack_file_path)?;
        Ok(pack_file_meta.len() >= self.max_pack_size_bytes.into())
    }
}
