use crate::error::AppError;
use crate::models::user::User;
use anyhow::Ok;
use sqlx::PgPool;
pub struct UserRepo;

impl UserRepo {
    pub async fn get_users(pool: &PgPool) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as!(User, r#"SELECT * FROM rust."users""#)
            .fetch_all(pool)
            .await
            .map_err(AppError::from)?;

        Ok(users)
    }

    pub async fn get_user(pool: &PgPool, id: i32) -> anyhow::Result<User> {
        let result = sqlx::query_as!(User, r#"SELECT * FROM rust."users" WHERE id = $1"#, id)
            .fetch_one(pool)
            .await
            .map_err(AppError::from)?;

        Ok(result)
    }

    pub async fn create_user(pool: &PgPool, user: &User) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO rust."users" (username,password) VALUES ($1, $2)"#,
            user.username,
            user.password
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}
