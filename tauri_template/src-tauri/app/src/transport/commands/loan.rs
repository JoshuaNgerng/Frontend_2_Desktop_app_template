use tauri::State;
use crate::generated::dto::loan::LoanRecordsDto;
use crate::repositories::loan::LoanRepository;
use crate::generated::schemas::request::LoanRecordQuery;
use crate::generated::schemas::response::{BatchHistoryItem, LoanStatsResponse};
use crate::schemas::response::listing_loans::{
    OffsetPaginatedResponse, LoanRecordsFilterOptions
};
use crate::utils::log::log_json;
use crate::{ SharedState, wait_until_ready };

#[tauri::command]
pub async fn get_upload_history(
    state: State<'_, SharedState>,
) -> Result<Vec<BatchHistoryItem>, String> {
    wait_until_ready().await;

    let app_state = state
        .get()
        .expect("READY=true but AppState missing");

    let pool = app_state.db.pool();

    let data = LoanRepository::get_upload_history(&pool).await
        .expect("cannot query batch history");

    Ok(data)
}

#[tauri::command]
pub async fn get_loans(
    state: State<'_, SharedState>,
    query: LoanRecordQuery
) -> Result<OffsetPaginatedResponse, String> {
    wait_until_ready().await;
    let app_state = state
        .get()
        .expect("READY=true but AppState missing");

    let pool = app_state.db.pool();

    let page = query.page;
    let limit = query.limit;
    
    println!("test before query {}, {}", page, limit);
    let query_result = match LoanRepository::get_loans(pool, query).await {
        Ok(v) => v,
        Err(e) => {
            let errmsg = format!("cannot query loan records: {}", e.to_string());
            println!("{}", &errmsg);
            return Err(errmsg);
        }
    };

    let data: Vec<_> = query_result.data.into_iter()
        .map(LoanRecordsDto::from).collect();
    
    let total_pages = (
        (query_result.count + limit as u64 - 1) as f64 / limit as f64
    ).ceil() as u32;
    
    println!("returning data {}, page {}", data.len(), total_pages);
    let res = OffsetPaginatedResponse{
        data,
        page,
        limit,
        total: query_result.count as u32,
        total_pages
    };
    // let _ = log_json(&res, "debug-loans.json");    

    Ok(res)
}

#[tauri::command]
pub async fn get_loans_filters(
    state: State<'_, SharedState>,
    // query: LoanRecordQuery
) -> Result<LoanRecordsFilterOptions, String> {
    wait_until_ready().await;

    let app_state = state
        .get()
        .expect("READY=true but AppState missing");

    let pool = app_state.db.pool();

    let filtered_data = LoanRepository::get_loans_filters(pool).await
        .expect("cannot query filter options");

    let res = LoanRecordsFilterOptions{
        vehicle_segment: filtered_data.iter().filter(|f| f.field == "vehicle_segment").map(|f| f.value.clone()).collect(),
        vehicle_make: filtered_data.iter().filter(|f| f.field == "vehicle_make").map(|f| f.value.clone()).collect(),
        dsr_band: filtered_data.iter().filter(|f| f.field == "dsr_band").map(|f| f.value.clone()).collect(),
        income_band: filtered_data.iter().filter(|f| f.field == "income_band").map(|f| f.value.clone()).collect(),
        scoring_decision_ori: filtered_data.iter().filter(|f| f.field == "scoring_decision_ori").map(|f| f.value.clone()).collect(),
        scoring_decision_simulated: filtered_data.iter().filter(|f| f.field == "scoring_decision_simulated").map(|f| f.value.clone()).collect(),
    };

    Ok(res)
}

#[tauri::command]
pub async fn get_loans_stats(
    state: State<'_, SharedState>,
) -> Result<LoanStatsResponse, String>{
    wait_until_ready().await;

    let app_state = state
        .get()
        .expect("READY=true but AppState missing");

    let pool = app_state.db.pool();
    let res = LoanRepository::get_stats(pool).await
        .expect("Cannot fetch Loan Stats");
    // let _ = log_json(&res, "debug-loan-stats.json");
    Ok(res)
}
