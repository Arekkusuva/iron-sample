extern crate iron;
extern crate time;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

mod api;
mod db;
mod utils;

const PORT: i32 = 8000;

// TODO: Parse config from env.
fn main() {
    println!("Start listening on port {}...", PORT);
    api::start_listening(PORT);
}
