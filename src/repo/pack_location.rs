use anyhow::{Context, Result};

pub struct ChunkLocation {
    pack_id: u32,
    offset: u64,
    length: u32,
}

impl ChunkLocation {
    pub fn to_bytes(&self) -> [u8; 16] {
        let mut buffer = [0u8; 16];
        buffer[0..4].copy_from_slice(&self.pack_id.to_le_bytes());
        buffer[4..12].copy_from_slice(&self.offset.to_le_bytes());
        buffer[12..16].copy_from_slice(&self.length.to_le_bytes());

        buffer
    }

    pub fn from_bytes(bytes_buffer: &[u8]) -> Result<Self> {
        let pack_id_bytes: [u8; 4] = bytes_buffer[0..4]
            .try_into()
            .context("Failed to deserialize pack_id from bytes")?;
        let pack_id = u32::from_le_bytes(pack_id_bytes);

        let offset_bytes: [u8; 8] = bytes_buffer[4..12]
            .try_into()
            .context("Failed to deserialize offset from bytes")?;
        let offset = u64::from_le_bytes(offset_bytes);

        let length_bytes: [u8; 4] = bytes_buffer[12..16]
            .try_into()
            .context("Failed to deserialize length from bytes")?;
        let length = u32::from_le_bytes(length_bytes);

        Ok(Self {
            pack_id: pack_id,
            offset: offset,
            length: length,
        })
    }
}
