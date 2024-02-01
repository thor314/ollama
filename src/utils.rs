use anyhow::{anyhow, Result};
use log::trace;

use crate::error::MyError;

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<(), MyError> {
  dotenv::dotenv().ok();
  // init_tracing();
  env_logger::init();
  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }

  Ok(())
}
