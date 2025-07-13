use serde::{de::DeserializeOwned, Serialize};
use std::{fs, path::Path};

pub fn load<T, P>(path: P) -> T
where
    T: Default + DeserializeOwned,
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .map(|content| serde_json::from_str(&content).unwrap_or(T::default()))
        .unwrap_or(T::default())
}

pub fn dump<T, P>(json: &T, path: P) -> bool
where
    T: Serialize,
    P: AsRef<Path>,
{
    let str = serde_json::to_string_pretty(json).unwrap();
    fs::write(path, str).is_ok()
}
