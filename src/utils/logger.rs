use std::io::stdout;
use std::env;
use std::sync::Mutex;

use slog::*;
use slog_term;
use slog_json::Json;

pub fn logger_factory() -> Logger {
    match env::var("LOG_OUTPUT") {
        Ok(log_output_type) => if log_output_type == "json" { output_json() } else { output_default() },
        _ => {
            output_default()
        },
    }
}

fn output_default() -> Logger {
    println!("Log output: default");
    let plain = slog_term::PlainSyncDecorator::new(stdout());
    let drain = slog_term::FullFormat::new(plain).build().fuse();

    Logger::root(
        drain,
        o!("app" => "web_sample")
    )
}

fn output_json() -> Logger {
    println!("Log output: json");
    let json = Mutex::new(Json::default(stdout())).map(Fuse);

    Logger::root(
        json,
        o!("app" => "web_sample")
    )
}

