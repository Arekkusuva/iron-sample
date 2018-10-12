extern crate iron;
extern crate time;

mod api;

const PORT: i32 = 8000;

// TODO: Parse config from env.
fn main() {
    println!("Start listening on port {}...", PORT);
    api::start_listening(PORT);
}
