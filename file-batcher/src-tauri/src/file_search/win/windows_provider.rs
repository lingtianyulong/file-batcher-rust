use crate::file_search::filesystem_provider::FileSystemProvider;
use ntfs::{Ntfs, structured_values::NtfsFileName};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct WindowsProvider;

impl WindowsProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystemProvider for WindowsProvider {
    async fn initialize(&self) {
        log::info!("WindowsProvider initialize");
    }

    async fn watch_changes(&self) {
        log::info!("WindowsProvider watch changes");
    }

    async fn metadata(&self) {
        log::info!("WindowsProvider metadata");
        // let file = File::open(r"\\.\C:").unwrap();
        // let mut reader = BufReader::new(file);
        // let ntfs = Ntfs::new(&mut reader).unwrap();

        // let root = ntfs.root_directory(&mut reader).unwrap();
        // let attrs = root.attributes();
        // log::info!("WindowsProvider metadata: {:?}", attrs);
        // // for attr in attrs {
        //     log::info!("WindowsProvider metadata: {}", attr.name());
        // }
        // let entries = root.entries().unwrap();
        // for entry in entries {
        //     // log::info!("WindowsProvider metadata: {}", entry.name());
        // }

        // let mut buffer = [0; 1024];
        // reader.read(&mut buffer).unwrap();
        // log::info!(
        // "WindowsProvider metadata: {}",
        // String::from_utf8_lossy(&buffer)
        // );
    }
}
