use std::path::PathBuf;

use crate::source::{Source, self};

/// Status of a [Download].
pub enum Status {
    Idle,
    Running,
    Finished
}

/// A download.
pub struct Download {
    pub source: Source,
    pub output: PathBuf,

    pub status: Status,

    pub size: u128,
    pub downloaded: u128,
}

impl Download {

    pub fn new(uri: String) -> Self {
        let source = source::parse_source(uri);

        Self {
            source,
            output: source.file_name(),
            size: 0,
            downloaded: 0,
            status: Status::Idle
        }
    }

    /// Fetch info about the download, without downloading it.
    pub fn fetch() {}

    /// Calculates the percentage of how many bytes have been downloaded and how many bytes need to
    /// be downloaded.
    pub fn progress(&self) -> u8 {
        ((self.downloaded / self.size) * 100) as u8
    }
}
