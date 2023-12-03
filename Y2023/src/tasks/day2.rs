use crate::utils::{file_reader::Meta, errors::Fault};
use regex::Regex;

// ================ Cubes ================

#[derive(Debug, Copy, Clone)]
struct Cubes {
  red: u16,
  green: u16,
  blue: u16,
}

impl Cubes {
  pub fn new (set: &str) -> Result<Self, Fault> {
    let red_re: Regex = Regex::new(r"(\d+) red").unwrap();
    let blue_re: Regex = Regex::new(r"(\d+) blue").unwrap();
    let green_re: Regex = Regex::new(r"(\d+) green").unwrap();
    let reds = get_color(set, red_re)?;
    let greens = get_color(set, green_re)?;
    let blues = get_color(set, blue_re)?;

    Ok (Cubes {
      red: reds,
      green: greens,
      blue: blues,
    })
  }

  pub fn from_individuals (red: u16, green: u16, blue: u16) -> Self {
    Cubes { red, green, blue }
  }

  pub fn exceeds_maximum (&self, maximums: Cubes) -> bool {
    self.blue <= maximums.blue && self.red <= maximums.red && self.green <= maximums.green
  }

  pub fn empty () -> Self {
    Cubes { red: 0, green: 0, blue: 0 }
  }

  pub fn override_higher (&self, external: &mut Cubes) {
    if self.red > external.red {
      external.red = self.red
    }
    if self.blue > external.blue {
      external.blue = self.blue
    }
    if self.green > external.green {
      external.green = self.green
    }
  }
}

// ================ Game ================

#[derive(Clone)]
struct Game {
  id: u16,
  sets: Vec<Cubes>
}


impl Game {
  pub fn new (raw_game: &str) -> Result<Self, Fault> {
    let id = get_game_id(raw_game)?;
    let sets: Vec<Cubes> = raw_game.split(';').into_iter().map(|s| Cubes::new(s).unwrap()).collect();

    Ok(Game {
      id,
      sets
    })
  }

  pub fn check_maximum_cubes (&self) -> bool {
    let maximum = Cubes::from_individuals(12, 13, 14);

    self.sets.clone().into_iter().all(|s| s.exceeds_maximum(maximum))
  }

  pub fn get_min_required_cubes (&self) -> Cubes {
    let mut minimums = Cubes::empty();
    self.sets.iter().for_each(|s| s.override_higher(&mut minimums));
    minimums
  }
}

// ================ REGEX-UTIL ================

fn get_game_id (line: &str) -> Result<u16, Fault> {
  let re = Regex::new(r"^Game (\d+):").unwrap();
  let Some(id) = re.captures(line) else {
    return Err(Fault::D2GameIdNotFound);
  };

  Ok(id[1].parse::<u16>().or_else(|_| Err(Fault::ConversionError))?)
}

fn get_color (set: &str, re: Regex) -> Result<u16, Fault> {
  let caps = re.captures(set);

  match caps {
    Some(findings) => findings[1].parse::<u16>().or_else(|_| Err(Fault::ConversionError)),
    None => Ok(0)
  }
}

// ================ FN UTIL ================

fn part_1_get_valid_games (games: Vec<Game>) -> Vec<Game> {
  games.into_iter().filter(|game| game.check_maximum_cubes()).collect()
}

fn summarize_ids (games: Vec<Game>) -> Result<u16, Fault> {
  games.iter().map(|g| g.id).reduce(|c, n| c + n).ok_or_else(|| Fault::UnknownError)
}

fn part_2_get_sum_of_powers (games: Vec<Game>) -> u32 {
  games.iter().map(|g| g.get_min_required_cubes()).map(|c| Into::<u32>::into(c.blue * c.green * c.red)).reduce(|c, n| c + n).unwrap().into()
}

// ================ ENTRY ================

pub fn day_2 (meta: Meta) -> Result<i32, Fault> {
  let games_raw = meta.get_lines_from_file();
  let games: Vec<Game> = games_raw.into_iter().map(|g| Game::new(g).unwrap()).collect();
  let valid_games = part_1_get_valid_games(games.clone());
  let sum_of_valid_ids = summarize_ids(valid_games)?;

  println!("[DAY 2][PART 1] The sum of the valid ids is {sum_of_valid_ids}");

  let powers = part_2_get_sum_of_powers(games);

  println!("[DAY 2][PART 2] The sum of the power of all sets is {powers}");

  Ok(0)
}


#[cfg(test)]
mod tests {
  use crate::{utils::file_reader::Meta, tasks::day2::{part_1_get_valid_games, summarize_ids, Cubes}};

use super::{Game, part_2_get_sum_of_powers};
  const GAME: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
  const PART_1_EXAMPLE_GAME: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

  #[test]
  fn parse_raw_game_into_struct() {
    let parsed = Game::new(GAME).unwrap();

    assert_eq!(parsed.id, 1);
    assert_eq!(parsed.sets[0].blue, 3);
    assert_eq!(parsed.sets[0].red, 4);
    assert_eq!(parsed.sets[0].green, 0);
    assert_eq!(parsed.sets[1].red, 1);
    assert_eq!(parsed.sets[1].green, 2);
    assert_eq!(parsed.sets[1].blue, 6);
    assert_eq!(parsed.sets[2].green, 2);
    assert_eq!(parsed.sets[2].red, 0);
    assert_eq!(parsed.sets[2].blue, 0);
  }

  #[test]
  fn part_1_example_check () {
    let meta = Meta{ file: PART_1_EXAMPLE_GAME.to_string(), task: "2".to_string() };
    let raw_games = meta.get_lines_from_file();
    let games: Vec<Game> = raw_games.into_iter().map(|g| Game::new(g).unwrap()).collect();
    let valid_games = part_1_get_valid_games(games);
    let sum_of_valid_ids = summarize_ids(valid_games).unwrap();

    assert_eq!(sum_of_valid_ids, 8);
  }

  #[test]
  fn check_minimum_required_cubes_per_game () {
    let game = Game::new(GAME).unwrap();
    let min_required_cubes_in_game: Cubes = game.get_min_required_cubes();

    assert_eq!(min_required_cubes_in_game.red, 4);
    assert_eq!(min_required_cubes_in_game.green, 2);
    assert_eq!(min_required_cubes_in_game.blue, 6);
  }

  #[test]
  fn part_2_example_check () {
    let meta = Meta{ file: PART_1_EXAMPLE_GAME.to_string(), task: "2".to_string() };
    let raw_games = meta.get_lines_from_file();
    let games: Vec<Game> = raw_games.into_iter().map(|g| Game::new(g).unwrap()).collect();

    let powers = part_2_get_sum_of_powers(games);
    assert_eq!(powers, 2286);
  }
}