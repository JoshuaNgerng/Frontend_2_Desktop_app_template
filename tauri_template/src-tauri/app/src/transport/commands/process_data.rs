use tauri::State;
use polars::prelude::SerReader;
use tauri_plugin_dialog::DialogExt;
use crate::types::bulk_upload::BulkUploadFileResult;
use crate::{ SharedState, wait_until_ready };
// use polars::prelude::*;

#[tauri::command]
pub async fn process_csv(
    state: State<'_, SharedState>,
    app: tauri::AppHandle
) -> Result<BulkUploadFileResult, String> {
    wait_until_ready().await;

    let app_state = state
        .get()
        .expect("READY=true but AppState missing");

    let get_file = app
        .dialog()
        .file()
        .blocking_pick_file();

    let file = match get_file {
        Some(v) => v,
        None => {
            let msg = "File path not detected";
            return Err(msg.to_string());
        }
    };

    let file_path = file.into_path().expect("cannot resolve file path");

    let filename = &file_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("Failed to get filename")
        .to_string();

    println!("File path {}, {}", file_path.to_str().unwrap(), filename);

    let mut raw_df = polars::prelude::CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(file_path))
        .unwrap()
        .finish() 
        .map_err(|e| e.to_string())?;



    let mut df = match check_df {
        Ok(d) => d,
        Err(e) => {
            let errmsg = e.to_string();
            println!("error schema df {}", &e);
            return Err(errmsg);
        }
    };

    println!("Rows loaded: {}", df.height());

    // 3. INSERT INTO DB
    // insert_into_db(pool.inner(), &df).await?;
    
    result.map_err(|e| { println!("{}", e.to_string()); e.to_string() })
}

#[tauri::command]
pub async fn delete_batch(
    state: State<'_, SharedState>, upload_batch: String
) -> Result<u64, String> {
    wait_until_ready().await;

    let app_state = state
        .get()
        .expect("READY=true but AppState missing");

    let pool = app_state.db.pool();
    
    res.map_err(|e| e.to_string())
} 