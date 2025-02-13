use dropshot::{ConfigLogging, ConfigLoggingLevel, ServerBuilder};

use ness_console::api;
use ness_console::{Context, ConsoleImpl};

#[tokio::main]
async fn main() -> Result<(), String> {
    let config_logging = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    };
    let log = config_logging
        .to_logger("ness-console-api")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    println!("OpenAPI spec:");
    println!("{}", api::generate_openapi_spec());

    let console_api = api::console_api_mod::api_description::<ConsoleImpl>().unwrap();
    let server = ServerBuilder::new(console_api, Context::new(), log)
        .start()
        .map_err(|error| format!("failed to create server: {}", error))?;

    server.await
}
