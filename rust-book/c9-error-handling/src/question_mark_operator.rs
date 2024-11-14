use std::fs::File;
use std::io::{self, Read};

pub enum ReadFileErrorType {
  FileNotThere,
  IrrelevantError,
}

pub struct MyError {
  pub error_type: ReadFileErrorType,
  pub error_msg: String,
}

// Implementation necessary if we want to return Result<String, MyError> from a function and use the '?' operator to simplify syntax
impl std::convert::From<io::Error> for MyError {
  fn from(io_error: io::Error) -> Self {
    match io_error.kind() {
      io::ErrorKind::NotFound => {
        return MyError {
          error_msg: String::from("File not found"),
          error_type: ReadFileErrorType::FileNotThere
        }
      }
      _ => {
        return MyError {
          error_msg: String::from("Irrelevant error"),
          error_type: ReadFileErrorType::IrrelevantError
        }
      }
    }
  }
}

pub fn read_username_from_file_question_mark(file_name: &str) -> Result<String, MyError> {
  let mut username = String::new();

  File::open(file_name)?.read_to_string(&mut username)?;

  Ok(username)
}
