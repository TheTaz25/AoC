use super::TaskData;

fn part1(input: String, results: &mut TaskData) {
  let mul_regex = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
  let mut multiplications: Vec<(usize, usize)> = vec![];

  for (_, [num1, num2]) in mul_regex.captures_iter(input.as_str()).map(|c| c.extract()) {
    multiplications.push((num1.parse::<usize>().unwrap(), num2.parse::<usize>().unwrap()));
  }

  results.push_log(format!("Part 1 - Extracted {} valid multiplications", multiplications.len()));
  results.mark_step("(1) Extracted MUL Operations".to_string());

  let mut calculation_result = 0;

  for (left, right) in multiplications {
    calculation_result += left * right;
  }

  results.set_task_1(format!("Result of all Multiplications is: {}", calculation_result));
}

fn part2(input: String, results: &mut TaskData) {
  let mul_regex = regex::Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d{1,3},\d{1,3}\))").unwrap();
  let mut instructions: Vec<&str> = vec![];

  
  for (_, [capture]) in mul_regex.captures_iter(input.as_str()).map(|c| c.extract()) {
    instructions.push(capture);
  }

  results.push_log(format!("Part 2 - Extracted {} valid Instructions", instructions.len()));
  results.mark_step("(2) Extracted Instructions".to_string());

  let mut enabled = true;
  let mut multiplication = 0;

  for instruction in instructions {
    match instruction {
      "do()" => enabled = true,
      "don't()" => enabled = false,
      mul => {
        if enabled {
          if let [_, left, right] = mul.split(&['(', ')', ',',][..]).take(3).collect::<Vec<_>>()[..] {
            results.push_log(format!("Part 2 - Enabled => {left}, {right}"));

            multiplication += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
          }
        }
      } 
    }
  }
  
  results.mark_step("(2) Evaluation Done".to_string());
  results.set_task_2(format!("Result of Multiplications with enablement is: {}", multiplication));
}

pub fn execute_day(input: String) -> TaskData {
  let mut results = TaskData::default();

  results.mark_step(String::from("Formatting Done"));

  part1(input.clone(), &mut results);

  results.mark_step("Task 1 - Done".to_string());

  part2(input, &mut results);

  results
}