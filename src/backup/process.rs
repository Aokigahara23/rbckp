use anyhow::Result;
use std::io::Read;
use std::{fs::File, path::PathBuf};

use crate::{
    backup::cdc_chunker::CdcChunker,
    repo::{
        file_meta::{FileMetadata, MetaFileType},
        repo::Repository,
    },
};

pub struct RBackupProcess<'a> {
    repository: &'a Repository<'a>,
}

impl<'a> RBackupProcess<'a> {
    pub fn new(repository: &'a mut Repository) -> Self {
        Self {
            repository: repository,
        }
    }

    pub fn process_dir(&mut self, dir_path: &PathBuf) -> Result<()> {
        let mut buf = [0u8; 1024 * 64];

        for dir_entry in std::fs::read_dir(dir_path)? {
            let file_path = dir_entry?.path();
            let file_meta = FileMetadata::from_file(&file_path)?;

            match file_meta.file_type {
                MetaFileType::Directory => self.process_dir(&file_path)?,

                _ => {
                    loop {
                        let mut file_obj = File::open(file_path)?;
                        let n = file_obj.read(&mut buf)?;
                        if n == 0 {
                            break;
                        }

                        chunker.push_bytes(&buf[..n], |chunk| {
                            // here: hash chunk, store chunk, append chunk_id to file recipe, etc.
                        });
                    }
                }
            }
        }

        Ok(())
    }
}
