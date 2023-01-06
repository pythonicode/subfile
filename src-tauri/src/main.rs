#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod util;
mod path;
mod upload;

use std::{
    sync::{
      Arc, Mutex,
    },
    path::PathBuf
};

use chrono::{Utc, TimeZone};
use path::FileWatcher;
use reqwest::{Client, multipart::{Form, Part}};
use serde::{Serialize, Deserialize};
use tauri::{State, Manager};
use argon2::{self, Config};
use rand::{rngs::OsRng, RngCore};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Wallet {
    created_at: i64,
    id: String,
    key: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct API {
    token: String,
    expiry: i64
}

impl API {
  pub fn expired(&self) -> bool {
    let expiry = Utc.timestamp_opt(self.expiry, 0).unwrap();
      let now = Utc::now(); 
      return now > expiry;
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Settings {
    wallet: Option<Wallet>,
    api: Option<API>,
    password: Option<String>,
    path: PathBuf,
    mode: String
}

struct Storage {
  settings: Arc<Mutex<Settings>>,
  app_handle: Option<tauri::AppHandle>
}

const PREFERENCE_FILE: &'static str = "subfile.config";

impl Storage {
    fn store(&self) {
      let path = util::app_config_dir().join(PREFERENCE_FILE);
      let contents = serde_json::to_string(&self.get()).unwrap();
      std::fs::write(path, contents).expect("failed to write storage to path");
      if let Some(app_handle) = &self.app_handle {
        let _ = app_handle.emit_all("update:settings", self.get());
      }
    }
  
    fn load() -> std::result::Result<Storage, Box<dyn std::error::Error>> {
        let path = util::app_config_dir().join(PREFERENCE_FILE);
        let string = std::fs::read_to_string(path)?;
        let settings: Settings = serde_json::from_str(&string).map_err(|x| x.to_string()).expect("no config file found.");
        let result = Storage {
          settings: Arc::new(Mutex::new(settings)),
          app_handle: None
        };
        Ok(result)
    }

    fn get(&self) -> Settings {
      self.settings.lock().expect("mutex has been poisoned.").clone()
    }
}

impl Default for Storage {
    fn default() -> Self {
      Storage::load().unwrap_or_else(|_| {
        let settings = Settings {
          wallet: None,
          api: None,
          password: None,
          path: util::default_dir(),
          mode: "category".to_string()
        };
        let storage = Storage {
          settings: Arc::new(Mutex::new(settings)),
          app_handle: None
        };
        storage.store();
        storage
      })
    }
}

#[tauri::command]
fn read_settings(storage: State<'_, Storage>) -> Result<Settings, String> {
  Ok(storage.get())
}

// #[derive(Serialize, Deserialize)]
// struct Content {
//   name: String,
//   r#type: String,
//   size: u64,
//   #[serde(rename= "contId")]
//   cont_id: u32,
//   cid: Option<String>,
//   dir: String,
//   coluuid: String,
//   #[serde(rename= "updatedAt")]
//   updated_at: String
// }

#[tauri::command]
async fn read_content(dir: String, client: State<'_, Client>, storage: State<'_, Storage>) -> Result<serde_json::Value, String> {
  let token = match storage.get().api {
    Some(api) => match api.expired() {
        true => create_token(storage.clone()).await.expect("Failed to fetch API token.").token,
        false => api.token,
    },
    None => create_token(storage.clone()).await.expect("Failed to fetch API token.").token
  };
  let wallet = storage.get().wallet.expect("no wallet found for this user");
  let col_id = wallet.id;
  let col_key = wallet.key;
  let url = util::API::get_url(&format!("/content?coluuid={}&dir={}", col_id, dir));
  let result = client.get(url)
    .header("Authorization", format!("Bearer {}", token))
    .header("Key", col_key)
    .send().await.unwrap().json::<serde_json::Value>().await.expect("failed to parse request as JSON");
  Ok(result)
}

#[tauri::command]
async fn update_password(password: String, client: State<'_, Client>, storage: State<'_, Storage>) -> Result<(), String> {
  let pwd = password.as_bytes();
  let mut salt = [0u8; 32];
  OsRng.fill_bytes(&mut salt);
  let config = Config::default();
  let token = match storage.get().api {
    Some(api) => match api.expired() {
        true => create_token(storage.clone()).await.expect("Failed to fetch API token.").token,
        false => api.token,
    },
    None => create_token(storage.clone()).await.expect("Failed to fetch API token.").token
  };
  let wallet = storage.get().wallet.expect("no wallet found for this user");
  let col_id = wallet.id;
  let col_key = wallet.key;
  println!("Collection: {} | Token: {}", col_id, token);
  let dir = "/.subfile/password";
  let response = client.delete(util::API::get_url(&format!("/content?coluuid={}&dir={}", col_id, dir)))
    .header("Key", col_key)
    .send().await.unwrap().text().await;
  match response {
    Ok(response) => println!("{}", response),
    Err(err) => println!("{:?}", err)
  }
  let url = format!("https://upload.estuary.tech/content/add?coluuid={}&dir={}", col_id, dir);
  let hash = argon2::hash_raw(pwd, &salt, &config).unwrap();
  let part = Part::bytes(hash).file_name("password.hash");
  let form = Form::new().part("data", part);
  let result = client.post(url)
    .header("Authorization", format!("Bearer {}", token))
    .multipart(form)
    .send().await.unwrap().text().await;
  match result {
    Ok(result) => println!("{:?}", result),
    Err(err) => println!("{:?}", err)
  }
  storage.settings.lock().expect("mutex has been poisoned").password = Some(password);
  storage.store();
  Ok(())
}

#[tauri::command]
fn update_path(path: String, storage: State<'_, Storage>, watcher: State<'_, Arc<Mutex<FileWatcher>>>) {
  let old_path = storage.get().path;
  watcher.lock().expect("mutex has been poisoned").unwatch(old_path.to_str().unwrap()).expect("failed to unwatch directory");
  watcher.lock().expect("mutex has been poisoned").watch(&path).expect("failed to unwatch directory");
  storage.settings.lock().expect("mutex has been poisoned").path = PathBuf::from(path);
  storage.store();
}

#[tauri::command]
fn update_mode(mode: String, storage: State<'_, Storage>) {
  storage.settings.lock().expect("mutex has been poisoned").mode = mode;
  storage.store();
}

#[tauri::command]
async fn create_wallet(storage: State<'_, Storage>) -> Result<Wallet, String> {
    if let Some(wallet) = storage.settings.lock().expect("mutex has been poisoned").wallet.clone() {
        return Ok(wallet);
    }
    println!("{}", "Creating New Wallet...");
    let result = reqwest::get(util::API::get_url("/wallet"))
      .await.expect("api request unsuccessful.")
      .json::<Wallet>()
      .await.expect("endpoint has the wrong format");
    storage.settings.lock().expect("mutex has been poisoned").wallet = Some(result.clone());
    storage.store();
    println!("{}", "Wallet Created.");
    Ok(result)
}

#[tauri::command]
async fn create_token(storage: State<'_, Storage>) -> Result<API, String> {
    if let Some(api) = storage.settings.lock().expect("Mutex has been poisoned").api.clone() {
        if !api.expired() {
          return Ok(api);
        }
    }
    println!("{}", "Fetching new API key...");
    let result = reqwest::get(util::API::get_url("/key"))
    .await.expect("api request unsuccessful.")
    .json::<API>()
    .await.expect("endpoint has the wrong format");
    storage.settings.lock().expect("Mutex has been poisoned").api = Some(result.clone());
    storage.store();
    println!("{}", "New API key created.");
    Ok(result)
}

#[tauri::command]
fn delete_wallet(storage: State<'_, Storage>) {
  storage.settings.lock().expect("mutex has been poisoned").wallet = None;
  storage.settings.lock().expect("mutex has been poisoned").password = None;
  storage.store();
}


fn main() {
    tauri::Builder::default()
        .setup(|app| {
          // Setup Storage
          let mut storage = Storage::default();
          storage.app_handle = Some(app.handle());
          let settings = storage.get();
          let path = settings.path.clone();
          app.handle().manage(storage);
          let client = reqwest::Client::new();
          let uploader_client = client.clone();
          app.handle().manage(client);
          let uploader = Arc::new(Mutex::new(upload::Uploader::new(app.handle(), uploader_client)));
          app.handle().manage(Arc::clone(&uploader));
          let file_watcher = Arc::new(Mutex::new(path::FileWatcher::new(uploader)));
          file_watcher.lock().expect("mutex has been poisoned").watch(path.to_str().unwrap()).unwrap();
          app.handle().manage(file_watcher);
          Ok(())
        })
        .invoke_handler(tauri::generate_handler![upload::upload, read_settings, read_content, update_password, update_path, update_mode, create_wallet, create_token, delete_wallet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
