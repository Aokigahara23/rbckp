use anyhow::Result;
use clap::Parser;
use rbckp::{backup::process::RBackupProcess, repo::repo::Repository};
// use rbckp::repo::repo::Repository;

fn main() -> Result<()> {
    let settings = rbckp::config::Settings::new()?;
    let args = rbckp::args::Args::parse();

    let mut chunker = rbckp::backup::cdc_chunker::CdcChunker::new(
        settings.chunk_settings.min,
        settings.chunk_settings.avg,
        settings.chunk_settings.max,
    );

    let mut repository = Repository::new(args.repository_dir_path)?;

    let backup_process = RBackupProcess::new(&mut chunker);

    Ok(())
}
