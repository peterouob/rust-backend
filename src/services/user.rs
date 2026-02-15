use axum::extract::{Path, State};
use axum::http::{Response, StatusCode};
use axum::{http, Json};
use axum::response::IntoResponse;
use serde_json::json;
use sqlx::{PgPool, Pool};
use crate::repo::user::UserRepo;

pub struct UserService;

impl UserService {
    pub async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
        let result = UserRepo::get_users(&pool).await;
        match result {
            Ok(users) => (
                StatusCode::OK,
                Json(json!(users))
                ).into_response(),
            Err(e) => Json(json!(e.to_string())).into_response()
        }
    }

    pub async fn get_user(Path(id): Path<String>) -> String{
        String::from("Hello")
    }

    pub async fn create_user() {

    }
}
