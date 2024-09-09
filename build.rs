use std::{io::Result, process::Command};

use chrono::{DateTime, SecondsFormat, Utc};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let git_commit_cmd = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_commit_hash = String::from_utf8(git_commit_cmd.stdout).unwrap();
    let now: DateTime<Utc> = Utc::now() + chrono::Duration::hours(8);
    let build_date = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");

    println!("cargo:rustc-env=GIT_COMMIT_HASH={git_commit_hash}");
    println!("cargo:rustc-env=GIT_BUILD_DATE={build_date}");
    println!("cargo:rustc-env=CARGO_MAIN_PKG_NAME={pkg_name}");
    println!("cargo:rustc-env=CARGO_MAIN_PKG_VERSION={pkg_version}");

    if let Ok(v) = std::env::var("DEP_OPENSSL_VERSION_NUMBER") {
        let version = u64::from_str_radix(&v, 16).unwrap();

        if version >= 0x1010_1000 {
            println!("cargo:rustc-cfg=openssl111");
        }
    }

    Ok(())
}
