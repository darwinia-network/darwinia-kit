mod default_features;
pub use default_features::Checker as DefaultFeaturesChecker;

mod storage_prefix;
pub use storage_prefix::Checker as StoragePrefixChecker;

// --- crates.io ---
use structopt::StructOpt;
// --- ci-bot ---
use crate::AnyResult;

pub trait Check {
	fn check(&self) -> AnyResult<()>;
}

#[derive(Debug, StructOpt)]
pub enum Checker {
	DefaultFeatures(DefaultFeaturesChecker),
	StoragePrefix(StoragePrefixChecker),
}
impl Check for Checker {
	fn check(&self) -> AnyResult<()> {
		match self {
			Checker::DefaultFeatures(checker) => checker.check(),
			Checker::StoragePrefix(checker) => checker.check(),
		}
	}
}
