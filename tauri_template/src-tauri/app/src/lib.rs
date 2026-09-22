// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


mod state;
mod transport;
mod db;
mod config;
mod utils;
mod types;
mod schemas;
mod services;
mod repositories;
mod generated;

use tauri::Manager;
use tokio::sync::{
    Notify, OnceCell
};
use std::{path::PathBuf, sync::{
    Arc, atomic::{AtomicBool, Ordering}
}};
use std::str::FromStr;
use sqlx::sqlite::{SqliteConnectOptions};

use transport::commands::process_data::{
    process_csv, delete_batch
};
use transport::commands::loan::{
    get_loans,get_upload_history, 
    get_loans_filters, get_loans_stats
};
use config::db::DbConfig;
use db::{manager::DbManager, pool::create_pool, seeder::run_seeders};
use state::app_state::AppState;

static READY: AtomicBool = AtomicBool::new(false);
static READY_NOTIFY: Notify = Notify::const_new();

pub type SharedState = Arc<OnceCell<AppState>>;

async fn wait_until_ready() {
    //
    // Fast path:
    // already initialized
    //
    if READY.load(Ordering::Acquire) {
        return;
    }

    //
    // Wait for notification
    //
    READY_NOTIFY.notified().await;

    //
    // Extra safety
    //
    while !READY.load(Ordering::Acquire) {
        READY_NOTIFY.notified().await;
    }
}

async fn setup_app(db_dir: PathBuf) -> AppState {
    // #[cfg(debug_assertions)]
    // dotenvy::dotenv().ok();
    let db_path = db_dir.join("app.db");
    std::fs::create_dir_all(&db_dir)
        .expect(&format!("failed to make directory: {:?}", db_dir));
    println!("debug db path {}", db_path.display());
    let config = DbConfig {
        max_connections: 10, enable_logging: true,
        opts: SqliteConnectOptions::from_str(
            &db_path.to_string_lossy()
        ).expect("cannot get sqlite options")
        .create_if_missing(true)
    };
    let pool = create_pool(&config).await; 
    let db = DbManager::new(pool).await.expect("db initlization failed");
    let pool = create_pool(&config).await; 
    run_seeders(&pool).await.expect("db seeder failed");
    AppState { db }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let shared_state: SharedState =
        Arc::new(OnceCell::new());

    tauri::Builder::default()
        .manage(shared_state.clone())
        .setup(move |app| {
            let handle = app.handle();
            setup_app_wrapper(shared_state, handle)
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            process_csv, delete_batch, get_loans,
            get_upload_history, get_loans_filters, get_loans_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_app_wrapper(
    shared_state: SharedState,
    app: &tauri::AppHandle
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let shared_state = shared_state.clone();
    let db_dir = app.path()
        .app_data_dir()
        .expect("failed to get app data dir");

    tauri::async_runtime::spawn(async move {
        //
        // Build actual app state
        //
        let app_state = setup_app(db_dir).await;

        //
        // Initialize OnceCell
        //
        shared_state
            .set(app_state)
            .expect("AppState already initialized");

        //
        // Signal readiness
        //
        READY.store(true, Ordering::Release);

        READY_NOTIFY.notify_waiters();

        println!("APP READY");
    });

    Ok(())
}

/*
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            tauri::async_runtime::spawn(async move {
                let state = init_app_state().await;

                STATE.set(state.clone()).ok();

                handle.manage(state);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![process_csv])
        .run(tauri::generate_context!())
        .expect("error while running app");
*/