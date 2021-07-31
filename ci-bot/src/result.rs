// --- crates.io ---
pub use anyhow::Result;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
#[error("{0}")]
pub enum Error {
	Custom(&'static str),
}
