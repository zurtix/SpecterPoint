use crate::models::state::AppState;
use common::db::agent::agent_count;
use common::error::Result;

#[tauri::command]
pub async fn count_agents(state: tauri::State<'_, AppState>) -> Result<u64> {
    agent_count(state.pool.clone()).await
}
