pub fn show_build_info() {
    let git_commit = option_env!("GIT_COMMIT_HASH").unwrap_or("unknown");
    let build_date = option_env!("GIT_BUILD_DATE").unwrap_or("unknown");

    tracing::info!("Git Commit Hash: {}", git_commit);
    tracing::info!("Build Date: {}", build_date);
}
