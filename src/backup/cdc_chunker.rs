use std::collections::HashMap;

use blake3;

fn chunk_id_hash(chunk: &[u8]) -> String {
    blake3::hash(chunk).to_hex().to_string()
}

/// Content-Defined Chunking (CDC) demo using a simple "Gear" rolling hash.
///
/// Goal:
/// - Split a byte stream into chunks where boundaries depend on content, not position.
/// - This makes deduplication robust against insertions/deletions near the beginning.
///
/// How boundaries are chosen (after we reach `min_chunk_size`):
/// - We keep a rolling hash `rolling_hash`.
/// - We cut a chunk when the lowest N bits of `rolling_hash` are all zero:
///       (rolling_hash & boundary_bitmask) == 0
/// - If the rolling hash behaves "random enough", this happens with probability 1 / 2^N,
///   so the average chunk size is about 2^N bytes.
///
/// We also enforce:
/// - never cut before min_chunk_size
/// - always cut at max_chunk_size (forced)
pub fn chunk_bytes_cdc(
    data: &[u8],
    min_chunk_size: usize,
    target_avg_chunk_size: usize,
    max_chunk_size: usize,
) -> (Vec<Vec<u8>>, HashMap<String, Vec<Vec<u8>>>) {
    assert!(min_chunk_size > 0, "min must be > 0");
    assert!(
        min_chunk_size <= target_avg_chunk_size && target_avg_chunk_size <= max_chunk_size,
        "must satisfy min <= avg <= max"
    );

    // Choose N so that 2^N is close to target_avg_chunk_size.
    //
    // Example:
    //   target_avg_chunk_size = 2048
    //   log2(2048) = 11
    //   => probability of boundary ≈ 1/2^11
    //   => average chunk size ≈ 2^11 = 2048 bytes
    //
    // We do this with floats in the demo for readability.
    let approx_bits = (target_avg_chunk_size as f64).log2();

    // Round to nearest integer number of bits.
    let rounded_bits = approx_bits.round();

    // Clamp to a safe range for u32 bit operations:
    // - at least 1 bit (mask not zero)
    // - at most 31 bits (so (1u32 << bits) is valid)
    let boundary_bits: u32 = rounded_bits.clamp(1.0, 31.0) as u32;

    // boundary_bitmask has the lowest `boundary_bits` bits set to 1.
    //
    // Example boundary_bits = 5:
    //   boundary_bitmask = (1<<5)-1 = 31 = 0b00011111
    //
    // Then (rolling_hash & boundary_bitmask) == 0 means:
    //   "the lowest 5 bits are all zero"
    let boundary_bitmask: u32 = (1u32 << boundary_bits) - 1;

    // A 256-entry lookup table that maps each byte (0..255) to a "random-looking" u32.
    // This gives the rolling hash good mixing properties.
    let byte_to_random: [u32; 256] = make_gear_table();

    let mut chunks: Vec<Vec<u8>> = Vec::new();
    let mut chunk_map: HashMap<String, Vec<Vec<u8>>> = HashMap::new();

    // Start index of the current chunk inside `data`.
    let mut chunk_start_index: usize = 0;

    // Rolling hash state for the current chunk scan.
    let mut rolling_hash: u32 = 0;

    // Walk through every byte; decide where to cut.
    for (i, &byte) in data.iter().enumerate() {
        // "Gear" rolling hash update.
        //
        // The shift keeps history (older bytes still affect the hash, but fade over time),
        // and adding a per-byte random value injects entropy.
        rolling_hash = rolling_hash
            .wrapping_shl(1)
            .wrapping_add(byte_to_random[byte as usize]);

        // Current chunk length if we include this byte (i is inclusive).
        let current_chunk_len = i + 1 - chunk_start_index;

        // Rule 1: Never cut before minimum size.
        if current_chunk_len < min_chunk_size {
            continue;
        }

        // Rule 2: Cut if we see the boundary pattern (probabilistic).
        let boundary_pattern_hit = (rolling_hash & boundary_bitmask) == 0;

        // Rule 3: Always cut if we hit max size (forced boundary).
        let forced_cut = current_chunk_len >= max_chunk_size;

        if boundary_pattern_hit || forced_cut {
            // Emit chunk data[chunk_start_index..=i]
            let tmp_data = data[chunk_start_index..=i].to_vec();
            chunks.push(tmp_data.clone());
            chunk_map
                .entry(chunk_id_hash(&tmp_data))
                .or_insert_with(Vec::new)
                .push(tmp_data);

            // Start a new chunk after this byte.
            chunk_start_index = i + 1;
            rolling_hash = 0;
        }
    }

    // Emit tail chunk if any bytes are left.
    if chunk_start_index < data.len() {
        let tmp_data = data[chunk_start_index..].to_vec();
        chunks.push(tmp_data.clone());
        chunk_map
            .entry(chunk_id_hash(&tmp_data))
            .or_insert_with(Vec::new)
            .push(tmp_data);
    }

    (chunks, chunk_map)
}

/// Build a deterministic "random-looking" table for bytes 0..255.
///
/// In real backup tools, this is typically a hardcoded constant table.
/// For a demo, generating it deterministically is fine as long as it's stable.
/// If you change this table, chunk boundaries will change too.
fn make_gear_table() -> [u32; 256] {
    let mut table = [0u32; 256];

    // Simple deterministic PRNG (Linear Congruential Generator-ish).
    // Not cryptographic. It's just to get stable "randomish" constants.
    let mut x: u32 = 0x1234_5678;

    for i in 0..256 {
        x = x.wrapping_mul(1664525).wrapping_add(1013904223);
        table[i] = x ^ (x >> 16);
    }

    table
}
