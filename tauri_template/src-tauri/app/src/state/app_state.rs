use crate::db::manager::DbManager;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DbManager
}
