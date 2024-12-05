use super::TaskData;

// struct SortHelper {
//   ord: Vec<(usize, usize)>,
// }

// impl Default for SortHelper {
//   fn default() -> Self {
//     Self {
//       ord: vec![],
//     }
//   }
// }

// impl SortHelper {
//   pub fn set_ord(&mut self, order: Vec<usize>) {
//     self.ord = order.iter().enumerate().map(|(index, value)| (index, *value)).collect();
//   }

//   fn get_index(&self, item: usize) -> usize {
//     self.ord.iter().find(|(_, value)| *value == item).expect(format!("Expected to have item {} in ord {:?}", item, self.ord).as_str()).0
//   }

//   pub fn compare(&self, lhs: usize, rhs: usize) -> bool {
//     let left_index = self.get_index(lhs);
//     let right_index = self.get_index(rhs);

//     match (left_index as isize - right_index as isize) as isize {
//       0 => false,
//       less if less < 0 => true,
//       _ => false
//     }
//   }
// }

// fn generate_number_set(from: &Vec<&str>) -> Vec<usize> {
//   let mut number_set : Vec<usize> = vec![];

//   // We want to know what numbers we are working with today
//   from
//     .iter()
//     .map(|rule| rule.split('|').collect::<Vec<&str>>())
//     .flatten()
//     .map(|a_number| {
//       a_number.parse::<usize>().unwrap()
//     })
//     .for_each(|v| { 
//       if !number_set.contains(&v) {
//         number_set.push(v);
//       }
//     });

//   number_set
// }

// fn split_and_parse(rules: Vec<&str>) -> Vec<(usize, usize)> {
//   rules
//   .iter()
//   .map(|rule| rule.split('|').collect::<Vec<&str>>())
//   .map(|x| {
//     match x[..] {
//       [left, right] => (left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap()),
//       _ => panic!("expect rule-splits to be two seperate strings")
//     }
//   })
//   .collect()
// }

// fn get_rhs_set(from: &Vec<(usize, usize)>) -> Vec<usize> {
//   let mut rhs_set: Vec<usize> = vec![];

//   from.iter().map(|(_, right)| *right).for_each(|l| {
//     if !rhs_set.contains(&l) {
//       rhs_set.push(l);
//     }
//   });

//   rhs_set
// }

// fn get_set_difference(left: Vec<usize>, right: Vec<usize>) -> Option<usize> {
//   let filtered: Vec<&usize> = left.iter().filter(|l| !right.contains(l)).collect();
//   println!("{}", format!("left: {:?} ,right: {:?}", left, right));
//   if filtered.len() > 0 {
//     return Some(*filtered[0]);
//   }
//   return None;
// }

// fn understand_sorting_rules(rules: Vec<&str>, results: &mut TaskData) -> SortHelper {
//   let mut helper = SortHelper::default();
//   let mut number_set : Vec<usize> = generate_number_set(&rules);
//   let mut order: Vec<usize> = vec![];

//   results.push_log(format!("Ruleset includes following numbers: {:?}", number_set));

//   let mut mapped_rules: Vec<(usize, usize)> = split_and_parse(rules);

//   results.push_log(format!("Mapped rules to: {:?}", mapped_rules));

//   while !mapped_rules.is_empty() {
//     let rhs_set = get_rhs_set(&mapped_rules);

//     let cloned = number_set.clone();
//     let diff : Option<usize> = get_set_difference(cloned, rhs_set);

//     match diff {
//       Some(d) => {
//         number_set = number_set.iter().filter(|n| **n != d).map(|n| *n).collect();
    
//         order.push(d);
    
//         mapped_rules = mapped_rules.iter().filter(|rule| rule.0 != d).map(|(l, r)| (*l, *r)).collect();
//       }
//       None => {
//         panic!("{}", format!("No diff received!\ncurrent order is {:?} \nmapped rules are {:?}", order, mapped_rules));
//       }
//     }
//   }

//   order.push(number_set[0]);

//   results.push_log(format!("Based on the rules, the sorting order for the pages is: {:?}", order));

//   helper.set_ord(order);

//   helper
// }



// ======================

type Rules = Vec<String>;
type PrintJobs = Vec<u8>;

type RulesAndJobs = (Rules, Vec<PrintJobs>);

fn format_input(input: String, results: &mut TaskData) -> RulesAndJobs {
  let mut rules: Rules = vec![];
  let mut jobs: Vec<PrintJobs> = vec![];

  input.lines().for_each(|datum| {
    if datum.contains('|') {
      rules.push(datum.to_string());
    } else if datum != "" {
      jobs.push(datum.split(',').map(|v| v.parse::<u8>().unwrap()).collect());
    }
  });

  results.push_log(format!("Found {} rules and {} jobs", rules.len(), jobs.len()));

  (rules, jobs)
}

fn part1((rules, jobs): &RulesAndJobs, results: &mut TaskData) -> Vec<PrintJobs> {
  let mut incorrect_jobs: Vec<PrintJobs> = vec![];
  let mut sum: usize = 0;
  'jobber: for job in jobs {
    results.push_log(format!("Checking if Job {:?} is in order", job));
    let mut rev: Vec<u8> = job.clone();
    rev.reverse();

    let mut checked: Vec<u8> = vec![];

    for to_check in rev {
      for next in checked.iter() {
        // Construct reversed format, if this exists inside the rules, the job is not ordered
        let rule_to_check = format!("{}|{}", next, to_check);
        if rules.contains(&rule_to_check) {
          results.push_log(format!("{} not contained in ruleset!", rule_to_check));
          incorrect_jobs.push(job.to_vec());
          continue 'jobber;
        }
      }
      checked.push(to_check);
    }

    let middle_job = job.get(job.len()/ 2).unwrap();
    sum += *middle_job as usize;
    results.push_log(format!("Job: {:?} is in order", job));
  }

  results.set_task_1(format!("Sum of correctly ordered jobs is {}", sum));
  results.mark_step("Part 1 - Done".to_string());

  incorrect_jobs
}

// fn part2((rules, jobs_to_fix): &RulesAndJobs, results: &mut TaskData) {
//   for job in jobs_to_fix {
//     let mut fixed : Vec<u8> = vec![];

//     for page in job {

//     }
//   }
// }

// FOR FUTURE: THE LIST DOES NOT HAVE A SINGULAR END AND START, IT WOULD PROBABLY BE NECESSARY TO
// ONLY GET THE RULES FOR THE NUMBERS INSIDE THE CURRENT JOB TO CHECK AGAINST!?!

// fn get_rules_for_job<'a>(all_rules: &'a str, job: &'a Vec<usize>) -> Vec<&'a str> {
//   all_rules.lines().filter(|line| {
//     // let left_and_right job.iter().any(|j| line.contains(&j.to_string()))
//   }).collect()
// }

// fn part1(jobs: &Vec<Vec<usize>>, rules: &str, results: &mut TaskData) {
//   let mut ordered_jobs: Vec<Vec<usize>> = vec![];
//   let mut unordered_jobs: Vec<Vec<usize>> = vec![];

//   jobs.iter().for_each(|job| {
//     let sub_ruleset = get_rules_for_job(rules, job);
//     results.push_log(format!("sub_ruleset {:?} for job {:?}", sub_ruleset, job));

//     let sort_helper = understand_sorting_rules(sub_ruleset, results);

//     // if job.is_sorted_by(|a, b| sort_helper.compare(*a, *b,)) {
//     //   ordered_jobs.push(job.to_vec());
//     // } else {
//     //   unordered_jobs.push(job.to_vec());
//     // }
//   });

//   // let sum_middle_ordered: usize = ordered_jobs.iter()
//   //   .map(|job| *job.get(job.len() / 2).unwrap())
//   //   .sum::<usize>();

//   // results.set_task_1(format!("Sum of correctly ordered jobs is {}", sum_middle_ordered));
//   // results.mark_step("Check ordered jobs and SUM".to_string());
// }

pub fn execute_day(data: String) -> TaskData {
  let mut results = TaskData::default();

  // let rules: Vec<&str> = data.split("\n\n").collect();

  // match rules[..] {
  //   [rules, jobs] => {
  //     // let sort_helper = understand_sorting_rules(rules.lines().collect(), &mut results);
  //     let printing_jobs: Vec<Vec<usize>> = jobs.lines().map(|line| {
  //       line.split(',').map(|v| v.parse::<usize>().unwrap()).collect()
  //     }).collect();

  //     results.mark_step("Formatting Completed".to_string());

  //     part1(&printing_jobs, &rules, &mut results);
  //   },
  //   _ => panic!("Unable to split data, is data complete?")
  // }


  let rules_and_jobs = format_input(data, &mut results);

  results.mark_step("Formatting done".to_string());

  let incorrect_jobs = part1(&rules_and_jobs, &mut results);

  // part2(&(rules_and_jobs.0, incorrect_jobs), &mut results);

  results
}