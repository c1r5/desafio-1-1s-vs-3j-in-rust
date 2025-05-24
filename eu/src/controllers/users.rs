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

    let mut db_write = db.write().unwrap() ;
    
    for user in users {
        db_write.push(user);
    }
    
    "Users uploaded successfully".to_string()
}