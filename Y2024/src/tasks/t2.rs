use super::TaskData;

fn is_safe(level: &Vec<usize>) -> bool {
  let is_sorted = level.is_sorted() || level.is_sorted_by(|a,b| a >= b);

  let mut windows = level.windows(2);
  let mut is_gap_safe: bool = true;

  while let Some([first, second]) = windows.next() {
    let diff = first.abs_diff(*second);
    let is_safe = 0 < diff && diff < 4;
    if !is_safe {
      is_gap_safe = false;
      break;
    }
  }

  is_sorted && is_gap_safe
}

fn is_forgiving_safe(report: &Vec<usize>, results: &mut TaskData) -> bool {
  if is_safe(report) {
    return true
  }

  // Brute force error-case in order to find a solution that works
  // We remove each Element once and try safety again...
  for i in 0..report.len() {
    let mut possible_solution = report.clone();
    possible_solution.remove(i);

    if is_safe(&possible_solution) {
      results.push_log(format!("Task 2 - Report{:?} after application converted to {:?} is safe", report, possible_solution));
      return true
    }
  }

  results.push_log(format!("Task 2 - Report{:?} contains problematic data, fix not possible", report));
  false
}

fn part1(reports: &Vec<Vec<usize>>, results: &mut TaskData) {
  let mut safe_counter = 0;
  for report in reports {
    if is_safe(report) {
      safe_counter += 1;
    }
  }

  results.set_task_1(format!("Data contains {} safe Reports", safe_counter));
  results.mark_step("Task 1 - Complete".to_string());
}

pub fn part2(reports: &Vec<Vec<usize>>, results: &mut TaskData) {
  let count = reports
    .iter()
    .map(|report| is_forgiving_safe(report, results))
    .filter(|r| *r)
    .collect::<Vec<bool>>()
    .len();

  results.set_task_2(format!("Data contains {} safe reports with Problem Damper", count));
  results.mark_step("Task 2 - Complete".to_string());
}

pub fn execute_day(input: String) -> TaskData {
  let mut results = TaskData::default();
  let lines: Vec<Vec<usize>> = input
    .split('\n')
    .map(|l| 
      l.split_whitespace()
      .map(|v| v
        .parse::<usize>()
        .expect("Unable to convert to usize")).collect())
    .collect();

  results.mark_step("Formatting Complete".to_string());

  part1(&lines, &mut results);
  part2(&lines, &mut results);

  results
}