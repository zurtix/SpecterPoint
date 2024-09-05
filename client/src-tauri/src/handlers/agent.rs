use crate::models::state::AppState;
use common::db::agent::{agent_count, get_agents, upsert_agent};
use common::error::Result;
use common::models::agent::Agent;

#[tauri::command]
pub async fn count_agents(state: tauri::State<'_, AppState>) -> Result<u64> {
    agent_count(state.pool.clone()).await
}

#[tauri::command]
pub async fn check_agent(state: tauri::State<'_, AppState>, agent: Agent) -> Result<()> {
    upsert_agent(agent, state.pool.clone()).await
}

#[tauri::command]
pub async fn agents(state: tauri::State<'_, AppState>) -> Result<Vec<Agent>> {
    get_agents(state.pool.clone()).await
}
