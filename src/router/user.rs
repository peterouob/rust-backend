use axum::Router;
use axum::routing::{get, post};
use crate::services::user::{get_users,get_user,create_user};
use crate::state::State;

pub fn routes() -> Router<State> {
    Router::new()
        .route("/list",get(get_users))
        .route("/list/{id}",get(get_user))
        .route("/create",post(create_user))
}