use config::{Config, ConfigError, File};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ChunkSettings {
    pub min: usize,
    pub avg: usize,
    pub max: usize,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub chunk_settings: ChunkSettings,
    pub debug: bool,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_file = File::with_name("./settings.ini");
        let settings_builder = Config::builder().add_source(config_file).build()?;

        settings_builder.try_deserialize::<Settings>()
    }
}
