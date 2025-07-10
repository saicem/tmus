use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::sync::Mutex;
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Loadable<T>(Mutex<Option<T>>)
where
    T: Default + DeserializeOwned + Serialize;

impl<T> Loadable<T>
where
    T: Debug + Default + DeserializeOwned + Serialize + Clone,
{
    pub const fn new() -> Self {
        Self(Mutex::new(None))
    }

    pub fn load(&self, path: impl AsRef<Path>) {
        *self.0.lock().unwrap() = Some(load(path));
    }

    pub fn dump(&self, path: impl AsRef<Path>) -> bool {
        dump(self.0.lock().unwrap().as_ref().unwrap(), path)
    }

    pub fn get(&self) -> T {
        self.0.lock().unwrap().as_ref().unwrap().clone()
    }

    pub fn set(&self, config: &T) {
        *self.0.lock().unwrap() = Some(config.clone());
    }

    pub fn set_field(&self, f: fn(&mut T)) {
        f(self.0.lock().unwrap().as_mut().unwrap());
    }
}

pub fn load<T, P>(path: P) -> T
where
    T: Default + DeserializeOwned,
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .map(|content| serde_json::from_str(&content).unwrap_or(T::default()))
        .unwrap_or(T::default())
}

pub fn dump<T, P>(config: &T, path: P) -> bool
where
    T: Serialize,
    P: AsRef<Path>,
{
    let str = serde_json::to_string_pretty(config).unwrap();
    fs::write(path, str).is_ok()
}
