async fn part1(data: String) -> Result<String, String> {
  tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  Ok(format!("{}", data))
}

pub async fn execute_day(input: String) -> Result<String, String> {
  part1(input).await
}