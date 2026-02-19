use anyhow::Result;
use sled::Db;
use std::path::PathBuf;

use crate::{
    backup::cdc_chunker::CdcChunker, config::RepositorySettings, repo::pack_writer::PackWriter,
};

pub struct Repository<'a> {
    repo_path: PathBuf,
    pack_writer: PackWriter,
    settings: &'a RepositorySettings,
    index_db: Db,
}

impl<'a> Repository<'a> {
    pub fn new(repo_path: PathBuf, settings: &'a RepositorySettings) -> Result<Self> {
        let pack_writer = PackWriter::new(&repo_path, settings.max_chunk_pack_size_bytes)?;
        std::fs::create_dir_all(&repo_path)?;

        let index_path = repo_path.join("index.db");
        let global_index_db = sled::open(&index_path)?;

        Ok(Repository {
            repo_path: repo_path.clone(),
            pack_writer: pack_writer,
            settings: settings,
            index_db: global_index_db,
        })
    }

    pub fn write_chunk(&self, chunk: &[u8]) -> Result<()> {
        let chunk_hash = blake3::hash(chunk);
        let index_entry = self.index_db.get(chunk_hash.as_bytes())?;

        Ok(())
    }
}
