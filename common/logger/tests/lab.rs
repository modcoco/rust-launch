#[test]
fn logs_file() {
    let (_handle, _guard) = logger::logger_trace::init_logger();
    tracing::info!("Client test");
}
