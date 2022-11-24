use dw_client::LogHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let log_handler = LogHandler::new(
        String::from("127.0.0.1:9000"),
        String::from("./test.log"),
        String::from("test_db"),
    )?;
    let _ = log_handler.start().await?;

    #[allow(unreachable_code)]
    Ok(())
}
