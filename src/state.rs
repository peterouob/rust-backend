use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone)]
pub struct State {
    pub db: PgPool,
}

impl State {
    pub fn new_db(db: PgPool) -> Self {
        Self {db}
    }
}

impl FromRef<State> for PgPool {
    fn from_ref(state: &State) -> Self {
        state.db.clone()
    }
}