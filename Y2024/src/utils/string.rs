pub fn split_by_space_to_tuple(datum: &str) -> (&str, &str) {
  let splits: Vec<&str> = datum.split_whitespace().collect();
  (splits[0], splits[1])
}

pub fn split_by_space_to_vec(line: &str) -> Vec<&str> {
  line.split_whitespace().collect()
}