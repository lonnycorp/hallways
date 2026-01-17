use std::fs;
use std::io::Read;

use url::Url;

#[derive(Debug)]
pub enum FetchError {
    HTTP(Box<ureq::Error>),
    IO(std::io::Error),
    URLJoin(url::ParseError),
    InvalidScheme,
}

pub fn fetch(base_url: &Url, href: &str) -> Result<Vec<u8>, FetchError> {
    let url = base_url.join(href).map_err(FetchError::URLJoin)?;

    return match url.scheme() {
        "http" | "https" => {
            let response = ureq::get(url.as_str())
                .call()
                .map_err(|err| FetchError::HTTP(Box::new(err)))?;
            let mut data = Vec::new();
            response
                .into_reader()
                .read_to_end(&mut data)
                .map_err(FetchError::IO)?;
            Ok(data)
        }
        "file" => {
            let path = url.to_file_path().map_err(|_| {
                FetchError::IO(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid file URL path",
                ))
            })?;
            let data = fs::read(&path).map_err(FetchError::IO)?;
            Ok(data)
        }
        _ => Err(FetchError::InvalidScheme),
    };
}
