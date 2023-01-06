use std::{path::Path, sync::{Arc, Mutex}};

use notify::{INotifyWatcher, Watcher, RecursiveMode, Event};
use tauri::{async_runtime::Receiver};

use crate::upload::Uploader;

pub struct FileWatcher {
    watcher: INotifyWatcher,
    uploader: Arc<Mutex<Uploader>>
}

impl FileWatcher {
    pub fn watch(&mut self, path: &str) -> Result<(), notify::Error> {
        self.watcher.watch(Path::new(path), RecursiveMode::Recursive)
    }

    pub fn unwatch(&mut self, path: &str) -> Result<(), notify::Error> {
        self.watcher.unwatch(Path::new(path))
    }
}

impl FileWatcher {
    pub fn new(uploader: Arc<Mutex<Uploader>>) -> Self {
        let (tx, mut rx) = tauri::async_runtime::channel::<Event>(100);
        
        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            let event = res.expect("Watcher failed to send update.");
            tauri::async_runtime::block_on(async {
                tx.send(event).await.unwrap();
            });
        });

        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                println!("{:?}", event)
            }
        });
        
        FileWatcher { 
            watcher: watcher.expect("Could not created file watcher."),
            uploader
        }
    }
}