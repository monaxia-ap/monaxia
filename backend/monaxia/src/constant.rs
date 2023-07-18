pub const SOFTWARE_NAME: &str = "monaxia";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VERSION_TAG: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_COMMIT_HASH"));
