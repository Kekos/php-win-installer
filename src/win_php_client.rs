use crate::win_php_domain::ReleasesResponse;
use log::{debug, trace};
use reqwest::blocking::Client;
use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::Error as SerdeError;
use std::fs::File;
use std::io::Cursor;

const BASE_URL: &str = "https://windows.php.net/downloads/releases";
const RELEASES_JSON_URL: &str = "releases.json";

pub struct WinPhpClient {
    reqwest: Client,
}

#[derive(Debug)]
pub enum ClientError {
    Reqwest(ReqwestError),
    HttpError(StatusCode),
    Serde(SerdeError),
    IO(std::io::Error),
}

impl WinPhpClient {
    pub fn new() -> Self {
        Self {
            reqwest: Client::new(),
        }
    }

    pub fn get_releases(&self) -> Result<ReleasesResponse, ClientError> {
        let url = format!("{}/{}", BASE_URL, RELEASES_JSON_URL);

        debug!("GET {}", url);

        let response_result = self
            .reqwest
            .get(url)
            .header("User-Agent", "win_php_client/0.0")
            .send();

        if let Err(err) = response_result {
            return Err(ClientError::Reqwest(err));
        }

        let response = response_result.unwrap();

        debug!("Response code: {}", response.status());

        let json = response.text();
        if let Err(err) = json {
            return Err(ClientError::Reqwest(err));
        }

        let data = ReleasesResponse::from_json(json.unwrap().as_str());

        if let Err(err) = data {
            return Err(ClientError::Serde(err));
        }

        Ok(data.unwrap())
    }

    pub fn download_zip(&self, path: &str, file: &mut File) -> Result<(), ClientError> {
        let url = format!("{}/{}", BASE_URL, path);

        debug!("GET {}", url);

        let response_result = self
            .reqwest
            .get(url)
            .header("User-Agent", "win_php_client/0.0")
            .send();

        if let Err(err) = response_result {
            return Err(ClientError::Reqwest(err));
        }

        let response = response_result.unwrap();

        debug!("Response code: {}", response.status());

        if response.status() != StatusCode::OK {
            return Err(ClientError::HttpError(response.status()));
        }

        let bytes_result = response.bytes();
        if let Err(err) = bytes_result {
            return Err(ClientError::Reqwest(err));
        }

        let bytes = bytes_result.unwrap();

        trace!("Downloaded ZIP of {} bytes", bytes.len());

        let mut content = Cursor::new(bytes);

        if let Err(err) = std::io::copy(&mut content, file) {
            return Err(ClientError::IO(err));
        }

        Ok(())
    }
}
