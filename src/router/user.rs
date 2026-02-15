use axum::Router;
use axum::routing::{get, post};
use crate::services::user::UserService;
use crate::state::State;

pub fn routes() -> Router<State> {
    Router::new()
        .route("/list",get(UserService::get_users))
        .route("/list/{id}",get(UserService::get_user))
        .route("/create",post(UserService::create_user))
}