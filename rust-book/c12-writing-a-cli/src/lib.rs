use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.file_path)?;

  println!("*** Lines matching '{}'{}***", config.query, if config.ignore_case {" (case insensitive) "} else { " " });
  for line in search(&contents, &config) {
    println!("{line}");
  }
  println!("***");

  Ok(())
}

fn search<'a>(contents: &'a str, config: &Config) -> Vec<&'a str> {
  if config.ignore_case {
    return search_case_insensitive(&config.query, contents)
  }
  search_case_sensitive(&config.query, contents)
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, String> {
    if args.len() < 3 {
      let msg = format!("Not enough arguments. Minimum: 3. Received: {} ({:?})", args.len(), args);
      return Err(msg);
    }

    let query = args[1].clone();
    let file_path = args[2].clone();

    Config::validate_file_path(&file_path)?;

    let ignore_case = Config::ignore_case(args.split_at(3).1);

    return Ok(Config {
      query,
      file_path,
      ignore_case
    });
  }

  fn validate_file_path(file_path: &str) -> Result<(), String> {
    let file_data = fs::metadata(file_path);

    if file_data.is_err() {
      let msg = format!("Could not read file: {}\nError: {}", file_path, file_data.expect_err("").to_string());
      return Err(msg);
    }

    Ok(())
  }

  fn ignore_case(flags: &[String]) -> bool {
    let ignore_case_env = env::var("IGNORE_CASE").is_ok();
    let ignore_case_arg = flags.iter().any(|flag| flag == "--ignore-case");

    ignore_case_env || ignore_case_arg
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "Rust\n\
      safe, fast, productive.\n\
      Pick three.\n\
      Duct tape.\n\
      ";

    assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.\n\
      Trust me.\n\
      ";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}