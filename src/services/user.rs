use crate::models::user::User;
use crate::repo::user::UserRepo;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Form, Json};
use serde_json::json;
use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
        let result = UserRepo::get_users(&pool).await;
        match result {
            Ok(users) => (StatusCode::OK, Json(json!(users))).into_response(),
            Err(e) => Json(json!(e.to_string())).into_response(),
        }
    }

    pub async fn get_user(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
        let result = UserRepo::get_user(&pool, id).await;
        match result {
            Ok(user) => (StatusCode::OK, Json(json!(user))).into_response(),
            Err(e) => Json(json!(e.to_string())).into_response(),
        }
    }

    pub async fn create_user(
        State(pool): State<PgPool>,
        Form(user): Form<User>,
    ) -> impl IntoResponse {
        let result = UserRepo::create_user(&pool, &user).await;
        match result {
            Ok(_) => (
                StatusCode::CREATED,
                Json(json!({"message": "User created successfully"})),
            )
                .into_response(),
            Err(e) => Json(json!(e.to_string())).into_response(),
        }
    }
}
