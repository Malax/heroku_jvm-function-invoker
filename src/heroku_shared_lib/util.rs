use std::io::Read;
use std::path::Path;
use std::{fs, io};

use reqwest;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("HTTP error while downloading file: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("IO error while downloading file: {0}")]
    IoError(#[from] std::io::Error),
}

/// Downloads a file via HTTP to a local path
///
/// # Examples
/// ```
///use rust_cnb_starter::util::download_file;
///use rust_cnb_starter::util::sha256;
///
///download_file("https://example.com/", "result.bin").unwrap();
///assert_eq!(sha256("result.bin").unwrap(), "ea8fac7c65fb589b0d53560f5251f74f9e9b243478dcb6b3ea79b5e36449c8d9");
/// ```
pub fn download_file(
    uri: impl AsRef<str>,
    destination: impl AsRef<std::path::Path>,
) -> Result<(), DownloadError> {
    let response = reqwest::blocking::get(uri.as_ref())?;
    let mut content = io::Cursor::new(response.bytes()?);
    let mut file = fs::File::create(destination.as_ref())?;
    io::copy(&mut content, &mut file)?;

    Ok(())
}

/// Obtains the SHA256 checksum of a file
///
/// # Examples
/// ```
///use rust_cnb_starter::util::sha256;
///use std::fs::write;
///
///write("test.txt", "Hello World!").unwrap();
///let sha256_sum = sha256("test.txt").unwrap();
///assert_eq!(sha256_sum, "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069");
/// ```
pub fn sha256(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path.as_ref())?;
    let mut buffer = [0x00; 10 * 1024];
    let mut sha256: Sha256 = sha2::Sha256::default();

    let mut read = file.read(&mut buffer)?;
    while read > 0 {
        Digest::update(&mut sha256, &buffer[..read]);
        read = file.read(&mut buffer)?;
    }

    Ok(format!("{:x}", sha256.finalize()))
}
