pub const SOFTWARE_NAME: &str = "monaxia";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
// pub const VERSION_TAG: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_COMMIT_HASH"));

pub mod mime {
    pub const APPLICATION_ACTIVITY_JSON: &str = "application/activity+json";
    pub const APPLICATION_LD_JSON: &str = "application/ld+json";
}
