use logger::logger_trace::init_logger;

#[test]
fn logs_file() {
    let (_guard, _handle) = logger::logger_trace::init_logger("test_boot", true);
    tracing::info!("Client test");
}

#[allow(dead_code)]
#[test]
fn main() {
    let (_guard, _handle) = init_logger("test_boot", false);

    tracing::info!("Client test");

    // drop(guard);
    tracing::info!("This is not an example");
}
