use axum::{
    routing::{get, post}, 
    extract::{State, Path},
    Json, Router,
    http::StatusCode,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    path::PathBuf,
    fs,
};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Deserialize)]
struct SetRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct SetResponse {
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Database {
    data: HashMap<String, String>,
    // We skip serializing the path because we don't want the path
    // saved inside the JSON file itself!
    #[serde(skip)]
    path: PathBuf,
}

impl Database {
    fn new<P: Into<PathBuf>>(path: P) -> Self {
        Database{
            data: HashMap::new(),
            path: path.into(),
        }
    }

    fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.path, json)?;
        Ok(())
    }

    fn load<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        if !path.exists() {
           return Ok(Database::new(path));
        }
        
       let contents = fs::read_to_string(&path)?;
       let data = serde_json::from_str(&contents)?;
       Ok(Database{data, path})
    }
}

struct AppState {
    db: Mutex<Database>,
}

#[tokio::main]
async fn main() {
    let db = Database::load("storage.json".to_string()).unwrap_or_else(|_| {
        Database::new("storage.json".to_string())
    });

    let shared_state = Arc::new(AppState{
        db: Mutex::new(db),
    });

    let app = Router::new()
        .route("/", get(|| async {"VaultAPI is Online"}))
        .route("/get/{key}", get(get_handler))
        .route("/set", post(set_handler))
        .with_state(shared_state);
    

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on: http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_handler(State(state): State<Arc<AppState>>, Path(key): Path<String>) -> String {
    let db = state.db.lock().unwrap();
    db.get(&key).cloned().unwrap_or_else(|| "Not Found".to_string())
}

async fn set_handler(State(state): State<Arc<AppState>>, Json(payload): Json<SetRequest>) -> (StatusCode, Json<SetResponse>){
    let mut db = state.db.lock().unwrap();
    db.insert(payload.key, payload.value);
    match db.save() {
        Ok(_) => {
            (StatusCode::OK, Json(SetResponse {
                status: "Success".to_string(),
            }))
        },
        Err(e) => {
            eprintln!("Database save error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(SetResponse {
                status: "Failed to persist data".to_string(),
            }))
        }
    }
   
}