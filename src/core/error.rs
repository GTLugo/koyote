use thiserror::Error;

#[derive(Error, Debug)]
pub enum KoyoteError {
  // #[error("Application encountered a critical error! `{0}`")]
  // Critical(&'static str), // Abandon current frame
  #[error("{error:#}")]
  Fatal {
    // Quit program
    error: anyhow::Error,
    exit_code: i32,
  },
}

impl KoyoteError {
  pub fn fatal(
    error: anyhow::Error,
  ) -> Self {
    Self::Fatal {
      error,
      exit_code: 1,
    }
  }

  pub fn fatal_with_code(
    error: anyhow::Error,
    exit_code: i32,
  ) -> Self {
    Self::Fatal {
      error,
      exit_code,
    }
  }

  pub fn fatal_str(
    message: &'static str,
  ) -> Self {
    Self::Fatal {
      error: anyhow::anyhow!("{}", message),
      exit_code: 1,
    }
  }

  pub fn fatal_str_with_code(
    message: &'static str,
    exit_code: i32,
  ) -> Self {
    Self::Fatal {
      error: anyhow::anyhow!("{}", message),
      exit_code,
    }
  }
}

pub trait Required<T> {
  /// Wraps the error in an `AppError::Fatal` to represent the `Result` being `Ok` as essential
  fn required(self) -> anyhow::Result<T>;
}

impl<T> Required<T> for anyhow::Result<T> {
  fn required(self) -> anyhow::Result<T> {
    self.map_err(|err| anyhow::anyhow!(KoyoteError::fatal(err)))
  }
}