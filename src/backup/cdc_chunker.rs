use std::collections::HashMap;

use blake3;

// boundary_bitmask has the lowest `boundary_bits` bits set to 1.
//
// Example boundary_bits = 5:
//   boundary_bitmask = (1<<5)-1 = 31 = 0b00011111
//
// Then (rolling_hash & boundary_bitmask) == 0 means:
//   "the lowest 5 bits are all zero"
pub struct CdcChunker {
    min: usize,
    avg: usize,
    max: usize,
    rolling_hash: u32,
    boundary_bitmask: u32,
    byte_to_random: [u32; 256],
    buffer: Vec<u8>,
}

impl CdcChunker {
    pub fn new(min: usize, avg: usize, max: usize) -> Self {
        let boundary_bits = (avg as f64).log2().round().clamp(1.0, 31.0) as u32;
        let boundary_bitmask: u32 = (1u32 << boundary_bits) - 1;
        let byte_to_random: [u32; 256] = make_gear_table();

        Self {
            min: min,
            avg: avg,
            max: max,
            rolling_hash: 0,
            boundary_bitmask,
            byte_to_random,
            buffer: Vec::with_capacity(max),
        }
    }

    fn chunk_id_hash(chunk: &[u8]) -> String {
        blake3::hash(chunk).to_hex().to_string()
    }

    pub fn push<F: FnMut(&[u8])>(&mut self, input: &[u8], mut callback: F) {
        for &byte in input {
            self.buffer.push(byte);

            // "Gear" rolling hash update.
            //
            // The shift keeps history (older bytes still affect the hash, but fade over time),
            // and adding a per-byte random value injects entropy.
            self.rolling_hash = self
                .rolling_hash
                .wrapping_shl(1)
                .wrapping_add(self.byte_to_random[byte as usize]);

            let buffer_len = self.buffer.len();

            if buffer_len < self.min {
                // don't cut until min
                continue;
            }

            // Cut if we see the boundary pattern (probabilistic).
            let boundary_pattern_hit = (self.rolling_hash & self.boundary_bitmask) == 0;
            let forced = buffer_len >= self.max;

            if boundary_pattern_hit || forced {
                callback(&self.buffer);
                self.buffer.clear();
                self.rolling_hash = 0;
            }
        }
    }

    pub fn finish<F: FnMut(&[u8])>(&mut self, mut callback: F) {
        if !self.buffer.is_empty() {
            callback(&self.buffer);
            self.buffer.clear();
            self.rolling_hash = 0;
        }
    }
}

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
