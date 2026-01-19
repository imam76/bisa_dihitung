use serde::{Deserialize, Serialize};
use crate::database::Database;
use crate::repository::UserRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
}

#[tauri::command]
pub async fn create_user(
    db: tauri::State<'_, Database>,
    request: CreateUserRequest,
) -> Result<String, String> {
    let user = UserRepository::create(db.pool(), &request.name, &request.email)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("User created: {} ({})", user.name, user.email))
}

#[tauri::command]
pub async fn get_all_users(db: tauri::State<'_, Database>) -> Result<String, String> {
    let users = UserRepository::find_all(db.pool())
        .await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::to_string(&users).map_err(|e| e.to_string())?)
}

#[tauri::command]
pub async fn get_user(db: tauri::State<'_, Database>, id: i32) -> Result<String, String> {
    let user = UserRepository::find_by_id(db.pool(), id)
        .await
        .map_err(|e| e.to_string())?;

    match user {
        Some(u) => Ok(serde_json::to_string(&u).map_err(|e| e.to_string())?),
        None => Err("User not found".to_string()),
    }
}

#[tauri::command]
pub async fn update_user(
    db: tauri::State<'_, Database>,
    id: i32,
    request: UpdateUserRequest,
) -> Result<String, String> {
    let user = UserRepository::update(db.pool(), id, &request.name, &request.email)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("User updated: {} ({})", user.name, user.email))
}

#[tauri::command]
pub async fn delete_user(db: tauri::State<'_, Database>, id: i32) -> Result<String, String> {
    let deleted = UserRepository::delete(db.pool(), id)
        .await
        .map_err(|e| e.to_string())?;

    if deleted {
        Ok("User deleted successfully".to_string())
    } else {
        Err("User not found".to_string())
    }
}
