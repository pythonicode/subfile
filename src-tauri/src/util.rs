use tauri::{api::path};
use std::path::PathBuf;


const APP_NAME: &'static str = "subfile";

pub fn default_dir() -> PathBuf {
    let path = path::document_dir().unwrap_or_else(|| {
        path::data_dir().expect("No default directory found.")
    });    
    let path = path.join(APP_NAME);
    std::fs::create_dir_all(&path).unwrap();
    path
}

pub fn app_config_dir() -> PathBuf {
    let path = path::config_dir().expect("could not get config dir");
    let path = path.join(APP_NAME);
    std::fs::create_dir_all(&path).unwrap();
    path
}

const API_URL: &'static str = "https://subfile-sigma.vercel.app/api";
pub struct API {}

impl API {
    pub fn get_url(endpoint: &str) -> String {
        format!("{}{}", API_URL, endpoint)
    }
}

