use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{os::unix::fs::MetadataExt, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub enum MetaFileType {
    Symlink,
    RegularFile,
    Directory,
}

impl MetaFileType {
    pub fn from_std_file_type(file_type: &std::fs::FileType) -> Self {
        if file_type.is_dir() {
            Self::Directory
        } else if file_type.is_symlink() {
            Self::Symlink
        } else {
            Self::RegularFile
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileMetadata {
    path: PathBuf,
    pub file_type: MetaFileType,
    size: u64,
    mode: u32,
    mtime: u64,
    chunks_hashes: Vec<[u8; 32]>,
}

impl FileMetadata {
    pub fn from_file(file_path: &PathBuf) -> Result<Self> {
        let meta = std::fs::symlink_metadata(file_path)?;

        let file_type = MetaFileType::from_std_file_type(&meta.file_type());
        let size = meta.size();
        let mode = meta.mode();
        let mtime = meta.mtime();
        let chunks = Vec::new();

        Ok(Self {
            path: file_path.clone(),
            file_type: file_type,
            size: size,
            mode: mode,
            mtime: mtime.try_into()?,
            chunks_hashes: chunks,
        })
    }

    pub fn append_chunk_hash(&mut self, chunk_hash: &[u8; 32]) {
        self.chunks_hashes.push(chunk_hash.clone());
    }
}
