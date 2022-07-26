use sea_orm::{Database, DatabaseConnection, DbErr};

use tokio::sync::OnceCell;

pub mod json;
pub mod my_table;

static CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_conn() -> Result<&'static DatabaseConnection, DbErr> {
    CONN.get_or_try_init(|| async { Database::connect("sqlite:development.sqlite3").await })
        .await
}
