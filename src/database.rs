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
    username TEXT unique NOT NULL,
    password_hash TEXT NOT NULL,
    email TEXT UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS sessions (
    session_token TEXT PRIMARY KEY NOT NULL, -- UUID as PRIMARY KEY
    user_id INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, -- Auto-set on creation
    last_updated DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, -- Auto-set on creation, updated on activity
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
").execute(&db).await.unwrap();
}

#[derive(Clone, FromRow, Debug)]
pub struct DbUser{
    pub id: i64,
    pub username: String,
    pub password_hash:String,
    pub email: String,
    pub created_at: String,
}

pub async fn get_users() -> Vec<DbUser> {

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query_as::<_, DbUser>("SELECT * from users").fetch_all(&db).await.unwrap()
}
pub async fn get_user_by_username(username: &str) -> Option<DbUser> {

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query_as::<_, DbUser>("SELECT * from users where username = ?").bind(username).fetch_one(&db).await.ok()
}
pub async fn get_user_by_token(token: &str) -> Option<DbUser> {

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query_as::<_, DbUser>("
SELECT u.* from users u
join sessions s on s.user_id = u.id
where s.session_token = ?;
").bind(token).fetch_one(&db).await.ok()
}

pub async fn get_sessions() -> Vec<String> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let sessions = sqlx::query("select session_token from sessions").fetch_all(&db).await.unwrap();
    let sessions: Vec<String> = sessions.iter().map(|row| {
        row.get::<String, &str>("session_token")
    }).collect();
    sessions
}
pub async fn get_session(token: &str) -> Option<()>{
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query("select session_token from sessions where session_token = ?").bind(token).fetch_one(&db).await.ok().map(|_| ())
}

pub async fn add_session_token(token: &String, user_id: &i64) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query("insert into sessions (session_token, user_id) values (?, ?)").bind(token).bind(user_id).execute(&db).await.unwrap();
}

pub async fn add_user(username: &String, hash: &String) -> Result<(), String>{
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let res = sqlx::query("insert into users (username, password_hash) values (?, ?)").bind(username).bind(hash).execute(&db).await;
    match res {
        Ok(_r) => {Ok(())}
        Err(e) => {
            println!("{e:?}");
            Err(e.to_string())
        }
    }
}
