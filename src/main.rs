use std::{
    fs::{self, File},
    io::Write,
};

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    println!("Current dir: {}", cwd.display());

    let settings = rbckp::config::Settings::new()?;
    let args = rbckp::args::Args::parse();

    println!("Current settings: {:?}", settings);
    println!("Args: {:?}", args);

    let data = fs::read(&args.target_file)?;

    // For text files, smaller numbers make it easier to observe behavior.
    let min_chunk_size = settings.chunk_settings.min;
    let target_avg_chunk_size = settings.chunk_settings.avg;
    let max_chunk_size = settings.chunk_settings.max;

    let (chunks, chunk_map) = rbckp::backup::cdc_chunker::chunk_bytes_cdc(
        &data,
        min_chunk_size,
        target_avg_chunk_size,
        max_chunk_size,
    );

    println!("File: {}", args.target_file.display());
    println!("Total bytes: {}", data.len());
    println!("Chunks: {}", chunks.len());
    println!(
        "Params: min={} avg={} max={}",
        min_chunk_size, target_avg_chunk_size, max_chunk_size
    );
    println!();

    println!("Chunks total: {}", chunks.len());

    let mut out_file = File::create_new("./output.txt")?;
    for (idx, chunk) in chunks.iter().enumerate() {
        // Show a small preview (safe for text-ish input).
        let preview_len = chunk.len().min(60);
        let preview = String::from_utf8_lossy(&chunk[..preview_len])
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t");

        writeln!(
            out_file,
            "chunk {:>4}: {:>6} bytes | preview: \"{}{}\"",
            idx,
            chunk.len(),
            preview,
            if chunk.len() > preview_len { "…" } else { "" }
        )?;
    }

    for (k, v) in chunk_map.iter() {
        println!("Chunk [{}] - count {}", k, v.len());
    }

    Ok(())
}
