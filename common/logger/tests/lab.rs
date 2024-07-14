use tracing::info;

#[test]
fn logs_file() {
    logger::logger_trace::init_logger();
    info!("Client test")
}
