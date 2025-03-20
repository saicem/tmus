use std::fs;

use serde::{de::DeserializeOwned, Serialize};

// Define a trait for loading and dumping configuration
pub trait ConfigLoader {
    fn load<P: AsRef<std::path::Path>>(file_path: P) -> Self;
    fn dump<P: AsRef<std::path::Path>>(&self, file_path: P);
}

impl<T> ConfigLoader for T
where
    T: Default + Serialize + DeserializeOwned,
{
    fn load<P: AsRef<std::path::Path>>(file_path: P) -> Self {
        {
            fs::read_to_string(file_path)
                .map(|content| serde_json::from_str(&content).unwrap_or(Self::default()))
                .unwrap_or(Self::default())
        }
    }

    fn dump<P: AsRef<std::path::Path>>(&self, file_path: P) {
        {
            let str = serde_json::to_string_pretty(self).unwrap();
            fs::write(file_path, str).unwrap()
        }
    }
}