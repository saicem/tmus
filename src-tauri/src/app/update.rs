use crate::app::global::get_app_handle;
use serde::Serialize;
use std::env;
use tauri::ipc::Channel;
use tauri::{AppHandle, State};
use tauri_plugin_updater::{Update, Updater, UpdaterExt};
use tokio::sync::Mutex;
use url::Url;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Updater(#[from] tauri_plugin_updater::Error),
    #[error("there is no pending update")]
    NoPendingUpdate,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    Started {
        content_length: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    Progress {
        chunk_length: usize,
    },
    Finished,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    version: String,
    current_version: String,
}

#[tauri::command]
pub async fn fetch_update(
    _app: AppHandle,
    pending_update: State<'_, PendingUpdate>,
) -> Result<Option<UpdateMetadata>> {
    let update = get_updater().check().await?;
    let update_metadata = update.as_ref().map(|update| UpdateMetadata {
        version: update.version.clone(),
        current_version: update.current_version.clone(),
    });
    *pending_update.0.lock().await = update;
    Ok(update_metadata)
}

#[tauri::command]
pub async fn install_update(
    pending_update: State<'_, PendingUpdate>,
    on_event: Channel<DownloadEvent>,
) -> Result<()> {
    let Some(update) = pending_update.0.lock().await.take() else {
        return Err(Error::NoPendingUpdate);
    };
    let mut started = false;
    update
        .download_and_install(
            |chunk_length, content_length| {
                if !started {
                    let _ = on_event.send(DownloadEvent::Started { content_length });
                    started = true;
                }
                let _ = on_event.send(DownloadEvent::Progress { chunk_length });
            },
            || {
                let _ = on_event.send(DownloadEvent::Finished);
            },
        )
        .await?;
    Ok(())
}

pub struct PendingUpdate(pub Mutex<Option<Update>>);

fn get_updater() -> Updater {
    let mut updater_builder = get_app_handle()
        .updater_builder()
        .timeout(std::time::Duration::from_secs(10));
    if let Some(proxy) = get_system_proxy() {
        updater_builder = updater_builder.proxy(proxy);
    }
    updater_builder.build().unwrap()
}

#[cfg(windows)]
fn get_system_proxy() -> Option<Url> {
    if let Ok(proxy) = env::var("http_proxy").or_else(|_| env::var("HTTP_PROXY")) {
        return Url::parse(&proxy).ok();
    }
    static INTERNET_SETTINGS_PATH: &str =
        r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";
    if let Ok(registry_key) = RegKey::predef(HKEY_CURRENT_USER).open_subkey(INTERNET_SETTINGS_PATH)
    {
        if let Ok(proxy_server) = registry_key.get_value::<String, _>("ProxyServer") {
            if let Ok(proxy_url) = Url::parse(("http://".to_string() + &proxy_server).as_ref()) {
                return Some(proxy_url);
            }
        }
    }
    None
}
