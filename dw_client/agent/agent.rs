use clap::Parser;
use dw_client::{error::ClientError, LogHandler};

#[derive(Parser)]
struct AgentArgs {
    /// dw server address && port
    #[clap(short = 'a', long = "addr")]
    server_address: String,

    /// monitor metrics file path
    #[clap(short = 'f', long = "file")]
    log_file: String,

    /// env name
    #[clap(short = 'd', long = "database")]
    env_name: String,

    /// -- might be deleted, split database by date
    #[clap(long = "split")]
    split: bool,

    /// use local ip
    #[clap(long = "local")]
    local: bool,
}

fn format_env_name(input: String) -> Result<String, ClientError> {
    let mut input = input.clone();
    if !input.is_ascii() {
        return Err(ClientError::CustomError("env_name contain non ascii character".into()));
    }
    input = input
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();

    if input.len() > 52 {
        input = String::from(&input[0..25]) + "__" + &input[input.len() - 25..];
    }

    Ok(input)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = AgentArgs::parse();

    let server_address = args.server_address;
    let log_file = args.log_file;
    let env_name = format_env_name(args.env_name)?;

    let log_handler = LogHandler::new(server_address, args.local, log_file, env_name).await?;
    let _ = log_handler.start().await?;

    #[allow(unreachable_code)]
    Ok(())
}
