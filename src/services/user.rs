use axum::extract::Path;
use axum::Json;

pub async fn get_users() -> Json<String> {
    Json(String::from("Hello"))
}

pub async fn get_user(Path(id): Path<String>) -> String{
    String::from("Hello")
}

pub async fn create_user() {

}