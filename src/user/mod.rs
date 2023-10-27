pub mod db;

pub const USER_COOKIE: &str = "_fq_user";

const DEFAULT_USERNAME: &str = "user";

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}
