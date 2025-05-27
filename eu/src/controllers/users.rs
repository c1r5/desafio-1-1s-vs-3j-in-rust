use std::time::Instant;

use rocket::{form::Form, fs::TempFile, tokio::io::AsyncReadExt, State};

use crate::types::{UserDB, Usuario};

#[derive(FromForm)]
pub struct Upload<'r> {
    file: TempFile<'r>,
}

#[post("/users", data = "<upload>")]
pub async fn upload_users(upload: Form<Upload<'_>>, db: &State<UserDB>) -> String {
    let mut content = Vec::new();

    let file = upload.file.open().await;

    match file {
        Ok(mut file) => {
            if let Err(e) = file.read_to_end(&mut content).await {
                return format!("Failed to read file: {}", e);
            }
        }
        Err(e) => return format!("Failed to open file: {}", e),
    }

    let readable = String::from_utf8_lossy(&content);

    let users: Vec<Usuario> = match serde_json::from_str(&readable) {
        Ok(users) => users,
        Err(_) => return "Invalid JSON format".to_string(),
    };

    let mut db_write = db.write().unwrap();

    for user in users {
        db_write.push(user);
    }

    "Users uploaded successfully".to_string()
}

#[derive(FromForm)]
pub struct UserFilter {
    score: u32,
    active: bool,
}

#[derive(serde::Serialize)]
struct SearchResponse {
    results: Vec<Usuario>,
    elapsed: u128,
}

#[get("/superusers?<filter..>")]
pub async fn get_superusers(filter: UserFilter, db: &State<UserDB>) -> String {
    let db_read = db.read().unwrap();

    let now = Instant::now();

    let superusers: Vec<Usuario> = db_read
        .iter()
        .filter(|user| user.score >= filter.score && user.active == filter.active)
        .cloned()
        .collect();

    if superusers.is_empty() {
        return "No superusers found".to_string();
    }

    let response = SearchResponse {
        results: superusers,
        elapsed: now.elapsed().as_nanos(),
    };

    serde_json::to_string(&response).unwrap_or_else(|_| "Failed to serialize response".to_string())
}
