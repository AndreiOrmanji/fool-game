use sea_orm::DatabaseConnection;

mod handler;

pub use handler::ErrorResponder;

#[derive(Debug)]
pub struct AppState {
    conn: DatabaseConnection,
}

impl AppState {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn get_db_conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}
