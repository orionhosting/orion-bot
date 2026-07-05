fn main() {
    println!("cargo:rerun-if-changed=locales");

    println!("cargo:rerun-if-changed=../../.git/HEAD");
    if let Some(rev) = parse_git_rev() {
        println!("cargo:rustc-env=APP_REV={rev}");
    }
}

/// Try to get the Git rev.
fn parse_git_rev() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=9", "HEAD"])
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}
