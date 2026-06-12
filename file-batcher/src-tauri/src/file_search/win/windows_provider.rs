use crate::file_search::filesystem_provider::FileSystemProvider;
use ntfs::{Ntfs, structured_values::NtfsFileName};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, FileSystemProvider)]
pub struct WindowsProvider;

impl FileSystemProvider for WindowsProvider {
    async fn initialize(&self) {
        log::info!("WindowsProvider initialize");
    }

    async fn watch_changes(&self) {
        log::info!("WindowsProvider watch changes");
    }

    async fn metadata(&self) {
        log::info!("WindowsProvider metadata");
    }
}
