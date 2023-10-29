//! Entrypoint of docker log parser
mod parser;
mod controller;
mod config;

// use parser::parse_logs;
use controller::cli_controller;


fn main() -> Result<(), std::io::Error> {
    cli_controller()
}
