use std::{process::Command, str::from_utf8};

fn main() {
    println!("cargo:rerun-if-changed=./.git/index");

    let git_revparse = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("failed to fetch git information");
    let commit_hash = from_utf8(&git_revparse.stdout)
        .expect("invalid UTF-8 string")
        .trim();
    println!("cargo:rustc-env=GIT_COMMIT_HASH={commit_hash}");
}
