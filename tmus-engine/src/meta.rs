use crate::util::Timestamp;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineMetadata {
    pub version: String,
    pub first_start_time: Timestamp,
}

impl EngineMetadata {
    pub fn load_from_file(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let metadata: EngineMetadata = serde_json::from_str(&contents)?;
        Ok(metadata)
    }

    pub fn write_to_file(&self, file_path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

static ENGINE_METADATA: OnceLock<EngineMetadata> = OnceLock::new();

pub fn init_engine_metadata(metadata: EngineMetadata) -> &'static EngineMetadata {
    ENGINE_METADATA.get_or_init(|| metadata)
}

pub fn get_engine_metadata() -> Option<&'static EngineMetadata> {
    ENGINE_METADATA.get()
}
