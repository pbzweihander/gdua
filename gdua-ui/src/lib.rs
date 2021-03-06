#[macro_use]
extern crate stdweb;

pub mod app;
mod chart;
mod service;
mod tree;

pub use app::App;

use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FileEntry {
    pub path: std::path::PathBuf,
    pub size: u64,
}

js_deserializable!(FileEntry);

#[derive(Debug, Default)]
pub struct PartialEqMutex<T>(pub std::sync::Mutex<T>);

impl<T> PartialEq for PartialEqMutex<T> {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
