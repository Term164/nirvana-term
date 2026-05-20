use nirvana_core::api::domain::{Connection, ConnectionData};
use nirvana_core::api::NirvanaApi;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct GuiConnection {
    id: i64,
    name: String,
    #[serde(rename = "type")]
    connection_type: String,
    hostname: String,
    username: String,
}

#[derive(Deserialize)]
struct CreateConnectionInput {
    name: String,
    #[serde(rename = "type")]
    connection_type: String,
    hostname: String,
    username: String,
    token: String,
}

impl From<Connection> for GuiConnection {
    fn from(connection: Connection) -> Self {
        Self {
            id: connection.id,
            name: connection.name,
            connection_type: connection.kind,
            hostname: connection.host,
            username: connection.identity,
        }
    }
}

#[tauri::command]
fn get_app_info(app: tauri::AppHandle) -> serde_json::Value {
    let info = app.package_info();

    serde_json::json!({
        "name": info.name,
        "version": info.version.to_string(),
        "authors": info.authors,
        "description": info.description,
    })
}

#[tauri::command]
fn get_active_connection() -> Result<Option<GuiConnection>, String> {
    let api = NirvanaApi::new().map_err(|error| error.to_string())?;
    api.get_active_connection()
        .map(|connection| connection.map(GuiConnection::from))
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn create_connection(input: CreateConnectionInput) -> Result<GuiConnection, String> {
    let mut api = NirvanaApi::new().map_err(|error| error.to_string())?;
    let data = ConnectionData {
        name: input.name,
        kind: input.connection_type,
        host: input.hostname,
        identity: input.username,
        secret_store: "plaintext".to_string(),
        token: input.token,
    };

    api.test_connection_data(ConnectionData {
        name: data.name.clone(),
        kind: data.kind.clone(),
        host: data.host.clone(),
        identity: data.identity.clone(),
        secret_store: data.secret_store.clone(),
        token: data.token.clone(),
    })
    .map_err(|error| error.to_string())?;

    let connection = api
        .add_connection(data)
        .map_err(|error| error.to_string())?;
    api.set_active_connection(connection.id)
        .map_err(|error| error.to_string())?;

    Ok(connection.into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_active_connection,
            create_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
