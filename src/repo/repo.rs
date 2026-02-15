use anyhow::Result;
use sled::Db;
use std::path::PathBuf;

use crate::{
    backup::cdc_chunker::CdcChunker,
    repo::{pack_location::PackLocation, pack_writer::PackWriter},
};

pub struct Repository<'a> {
    repo_path: PathBuf,
    pack_writer: PackWriter,
    cdc_chunker: &'a CdcChunker,
    index_db: Db,
}

impl<'a> Repository<'a> {
    pub fn new(repo_path: PathBuf, cdc_chunker: &'a CdcChunker) -> Result<Self> {
        let pack_writer = PackWriter::new(&repo_path)?;
        std::fs::create_dir_all(&repo_path)?;

        let index_path = repo_path.join("index.db");
        let global_index_db = sled::open(&index_path)?;

        Ok(Repository {
            repo_path: repo_path.clone(),
            pack_writer: pack_writer,
            cdc_chunker: cdc_chunker,
            index_db: global_index_db,
        })
    }

    pub fn write_chunks(&self, chunks: &Vec<&[u8]>) -> Result<()> {
        for &chunk in chunks {
            let chunk_hash = blake3::hash(chunk);
            let index_entry = self.index_db.get(chunk_hash.as_bytes())?;
        }

        Ok(())
    }
}
