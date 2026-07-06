fn main() {
    // Locales
    println!("cargo:rerun-if-changed=locales");

    // Build time
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time went backwards somehow")
        .as_millis();

    println!("cargo:rustc-env=BUILD_TIME={now}");

    // Git revision
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    if let Some(rev) = parse_git_rev() {
        println!("cargo:rustc-env=APP_REV={rev}");
    }
}

/// Try to get the Git rev.
fn parse_git_rev() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout).ok()
    } else {
        None
    }
}
