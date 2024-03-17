use std::env;

mod funstack;
mod frontend;
mod cli;
mod io;

fn main() {
    if let Err(err) = funstack::run(env::args().collect()) {
        eprintln!("Error: {err}")
    }
}
