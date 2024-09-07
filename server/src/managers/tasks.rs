use common::models::task::Task;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

type TaskList = HashMap<String, Vec<Task>>;

#[derive(Clone, Default)]
pub struct TaskManager {
    tasks: Arc<Mutex<TaskList>>,
}

impl TaskManager {
    pub async fn add(&self, id: String, task: Task) -> usize {
        let mut tasks = self.tasks.lock().await;
        tasks.entry(id.clone()).or_default().push(task);

        match tasks.get(&id) {
            Some(t) => t.len(),
            _ => 0,
        }
    }

    pub async fn tasks(&self, id: String) -> Vec<Task> {
        match self.tasks.lock().await.get(&id) {
            Some(tasks) => tasks.clone(),
            _ => vec![],
        }
    }

    pub async fn complete(&self, id: String, task_idx: usize) {
        let mut tasks = self.tasks.lock().await;

        if let Some(list) = tasks.get_mut(&id) {
            list.remove(task_idx);

            if list.is_empty() {
                tasks.remove(&id);
            }
        }
    }
}
