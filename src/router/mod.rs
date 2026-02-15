pub mod user;

use axum::Router;
use crate::state::State;

pub fn routes() -> Router<State> {
    Router::new()
        .nest("/user", user::routes())
}