use std::fs::File;

mod question_mark_operator;
use question_mark_operator::ReadFileErrorType;

fn main() {
  println!("# Chapter 9: Error Handling");

  recoverable_error();

  propagate_errors();
}

fn recoverable_error() {
    println!("## Recoverable errors when reading a file");
    let file_name = "foo.bar.txt";
    let nonexistent_file = File::open(file_name);

    match nonexistent_file {
        Ok(_) => println!("File {file_name} exists"),
        Err(e) => println!("File {file_name} does not exist. Error: {e}"),
    }
    println!("Program continues running even if file doesn't exist...");
}

fn propagate_errors() {
  println!("## Propagating errors with Result<T,E>");
  let username_result = question_mark_operator::read_username_from_file_question_mark("foo.bar.txt");

  println!("### Handling self-made Error types with 'match' (see code)");
  match username_result {
    Ok(username) => println!("Username in foo.bar.txt: '{username}'"),
    Err(e) => {
      match e.error_type {
        ReadFileErrorType::FileNotThere => {
          println!("File foo.bar.txt does not exist");
        }
        ReadFileErrorType::IrrelevantError => {
          println!("Not able to read username from foo.bar.txt");
        }
      }
    }
  }
}
