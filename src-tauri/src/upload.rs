use std::{path::PathBuf, sync::{Arc, Mutex}};
use tauri::{AppHandle, async_runtime::Sender, Manager, State};

use crate::{Storage, create_token};

pub struct Uploader {
    pub sender: Sender<PathBuf>,
    pub client: reqwest::Client
}

#[tauri::command]
pub fn upload(path: PathBuf, uploader: State<'_, Arc<Mutex<Uploader>>>) {
    uploader.lock().expect("Mutex is poisoned.").upload(path);
}

async fn upload_to_estuary(app_handle: &AppHandle, client: &reqwest::Client, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let _ = app_handle.emit_all("upload:start", path.clone());
    let token = match app_handle.state::<Storage>().get().api {
        Some(api) => match api.expired() {
            true => create_token( app_handle.state::<Storage>()).await.expect("failed to fetch API token.").token,
            false => api.token,
        },
        None => create_token( app_handle.state::<Storage>()).await.expect("failed to fetch API token.").token
    };
    println!("{:?}", path);
    // let response = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send()
    //     .await?;
    Ok(())
}

impl Uploader {
    pub fn new(app_handle: AppHandle, client: reqwest::Client) -> Uploader {
        let (sender, mut receiver) = tauri::async_runtime::channel::<PathBuf>(65536);
        let async_client = client.clone();
        tauri::async_runtime::spawn(async move {
            while let Some(path) = receiver.recv().await {
                upload_to_estuary(&app_handle, &async_client, path.clone()).await.unwrap_or_else(|_| {
                    let _ = &app_handle.emit_all("upload:failed", path);
                });
            }
        });
        Uploader {
            sender,
            client
        }
    }
}

impl Uploader {
    pub fn upload(&mut self, path: PathBuf) {
        let sender = self.sender.clone();
        tauri::async_runtime::spawn(async move {
            sender.send(path).await.expect("Failed to queue upload.");
        });        
    }
}