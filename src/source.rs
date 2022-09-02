use url::Url;

/// Source that can be used to download a file.
pub enum Source {
    Http(String)
}

impl Source {
    pub fn file_name(&self) -> String {
        match self {
            Source::Http(url) => {
                let url = Url::parse(url)
                    .expect("couldn't parse URL");

                let file = url.path_segments().unwrap()
                    .last()
                    .unwrap();

                file.to_string()
            },
        }
    } 
}

/// Parses a URI into a [Source].
pub fn parse_source(uri: String) -> Source {
    // temporary, of course
    Source::Http(uri)
}
