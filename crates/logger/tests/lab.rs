use logger::logger_trace::init_logger;

#[test]
fn logs_file() {
    let (_handle, _guard, _file_hander) = logger::logger_trace::init_logger("test_boot", true);
    tracing::info!("Client test");
}

#[allow(dead_code)]
#[test]
fn main() {
    let (_handle, _guard, _file_hander) = init_logger("test_boot", false);

    tracing::info!("Client test");

    // drop(guard);
    tracing::info!("This is not an example");
}
