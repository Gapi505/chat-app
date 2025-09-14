use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool, Row};

pub const DB_URL: &str = "sqlite://sqlite.db";
pub async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let _result = sqlx::query("
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(32) NOT NULL,
    password VARCHAR(32) NOT NULL
);
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY NOT NULL,
    session_token STRING NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
").execute(&db).await.unwrap();
}

#[derive(Clone, FromRow, Debug)]
pub struct DbUser{
    pub id: i64,
    pub username: String,
    pub password: String
}

pub async fn get_users() -> Vec<DbUser> {

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query_as::<_, DbUser>("SELECT * from users").fetch_all(&db).await.unwrap()
}
pub async fn get_sessions() -> Vec<String> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let sessions = sqlx::query("select session_token from sessions").fetch_all(&db).await.unwrap();
    let sessions: Vec<String> = sessions.iter().map(|row| {
        row.get::<String, &str>("session_token")
    }).collect();
    sessions
}

pub async fn add_session_token(token: &String, user_id: &i64) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query("insert into sessions (session_token, user_id) values (?, ?)").bind(token).bind(user_id).execute(&db).await.unwrap();
}
