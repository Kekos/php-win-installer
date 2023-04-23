use crate::win_php_domain::ReleasesResponse;
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
        let response = self
            .reqwest
            .get(format!("{}{}", BASE_URL, RELEASES_JSON_URL))
            .header("User-Agent", "win_php_client/0.0")
            .send();
        if let Err(err) = response {
            return Err(ClientError::Reqwest(err));
        }

        let json = response.unwrap().text();
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

        let response_result = self
            .reqwest
            .get(url)
            .header("User-Agent", "win_php_client/0.0")
            .send();

        if let Err(err) = response_result {
            return Err(ClientError::Reqwest(err));
        }

        let response = response_result.unwrap();

        if response.status() != StatusCode::OK {
            return Err(ClientError::HttpError(response.status()));
        }

        let bytes = response.bytes();
        if let Err(err) = bytes {
            return Err(ClientError::Reqwest(err));
        }

        let mut content = Cursor::new(bytes.unwrap());

        if let Err(err) = std::io::copy(&mut content, file) {
            return Err(ClientError::IO(err));
        }

        Ok(())
    }
}
