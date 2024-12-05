use tokio::time::Instant;

pub mod t0;
pub mod t1;
pub mod t2;
pub mod t3;
pub mod t4;
pub mod t4_hashmap;
pub mod t5;

#[derive(Clone)]
pub struct TaskData {
  result_task_1: Option<String>,
  result_task_2: Option<String>,
  log: Vec<String>,
  timestamps: Vec<(String, Instant)>,
}

impl Default for TaskData {
  fn default() -> Self {
    Self {
      result_task_1: None,
      result_task_2: None,
      log: vec![],
      timestamps: vec![(String::from("Start"),
 tokio::time::Instant::now())]
    }
  }   
}

impl TaskData {
  pub fn set_task_1(&mut self, result: String) {
    self.result_task_1 = Some(result);
  }
  pub fn set_task_2(&mut self, result: String) {
    self.result_task_2 = Some(result);
  }

  pub fn push_log(&mut self, line: String) {
    self.log.push(line);
  }

  pub fn mark_step(&mut self, name: String) {
    self.timestamps.push((name, tokio::time::Instant::now()));
  }

  pub fn get_timestamps(&self) -> Vec<(String, Instant)> {
    self.timestamps.clone()
  }

  pub fn get_results(&self) -> (String, String) {
    (
      self.result_task_1.clone().unwrap_or("Not Calculated".to_string()),

      self.result_task_2.clone().unwrap_or("Not Calculated".to_string())
    )
  }

  pub fn get_logs(&self) -> Vec<String> {
    self.log.clone()
  }
}