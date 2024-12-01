pub fn split_by_space(datum: &str) -> (&str, &str) {
  let splits: Vec<&str> = datum.split_whitespace().collect();
  (splits[0], splits[1])
}