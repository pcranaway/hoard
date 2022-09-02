use std::path::PathBuf;

/// Source that can be used to download a file.
pub enum Source {
    Http(String)
}

/// Status of a [Download].
pub enum Status {
    Idle,
    Running,
    Finished
}

/// A download.
pub struct Download {
    pub sources: Vec<Source>,
    pub output: PathBuf,

    pub status: Status,

    pub size: u128,
    pub downloaded: u128,
}

impl Download {
    /// Calculates the percentage of how many bytes have been downloaded and how many bytes need to
    /// be downloaded.
    pub fn progress(&self) -> u8 {
        ((self.downloaded / self.size) * 100) as u8
    }
}
