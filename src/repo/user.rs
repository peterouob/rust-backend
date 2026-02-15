use axum::extract::State;
use sqlx::PgPool;
use crate::error::AppError;
use crate::models::user::User;
pub struct UserRepo;

impl UserRepo {
    pub async fn get_users(pool: &PgPool) -> Result<User, AppError>{
        let users = sqlx::query_as!(
            User,
            "SELECT * FROM users"
        )
            .fetch(pool)
            .await?;

        Ok(users)
    }
    pub fn get_user() {}

    pub async fn create_user(State(pool): State<PgPool>) {

    }
}