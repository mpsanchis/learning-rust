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
  contents
    .lines()
    .filter(|&line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();

  contents
    .lines()
    .filter(|&line| line.to_lowercase().contains(&query))
    .collect()
}

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    // skip first arg: name of the executable
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err(String::from("Didn't get a query string (first argument)"))
    };

    let file_path = match args.next() {
      Some(arg) => arg,
      None => return Err(String::from("Didn't get a filepath string (second argument)"))
    };
    Config::validate_file_path(&file_path)?;

    // After calls to "next()", the return of "collect()" is the remaining items not yet iterated
    let flags: Vec<_> = args.collect();

    let ignore_case = Config::ignore_case(&flags);

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

  fn ignore_case(flags: &Vec<String>) -> bool {
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