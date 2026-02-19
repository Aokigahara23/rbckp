use config::{Config, ConfigError, File};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ChunkSettings {
    pub min: usize,
    pub avg: usize,
    pub max: usize,
}

impl ChunkSettings {
    pub fn default() -> Self {
        Self {
            min: 256 * 1024,
            avg: 1 * 1024 * 1024,
            max: 4 * 1024 * 1024,
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct RepositorySettings {
    pub max_chunk_pack_size_bytes: u64,
}

impl RepositorySettings {
    fn default() -> Self {
        Self {
            max_chunk_pack_size_bytes: 1u64 << 30, // 1 GiB,
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub chunk_settings: ChunkSettings,
    pub repository_settings: RepositorySettings,

    pub debug: bool,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            chunk_settings: ChunkSettings::default(),
            repository_settings: RepositorySettings::default(),
            debug: false,
        }
    }

    pub fn new() -> Result<Self, ConfigError> {
        let config_file = File::with_name("./settings.ini");
        match Config::builder().add_source(config_file).build() {
            Ok(settings_builder) => settings_builder.try_deserialize::<Settings>(),
            _ => Ok(Self::default()),
        }
    }
}
