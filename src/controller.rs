extern crate clap;

use clap::{App, Arg};
use crate::parser::parse_logs;

pub fn cli_controller() -> Result<(), std::io::Error> {
    let matches = App::new("Docker Log Parser")
        .version("0.1")
        .author("Ruslan Tahiiev")
        .about("Parse docker logs")
        .arg(Arg::with_name("app_name")
            .short("n")
            .long("name")
            .value_name("app")
            .help("App name")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("/path/where/execute")
            .help("Path where command must be executed")
            .required(false)
            .takes_value(true))
        .get_matches();

    let app_name = matches.value_of("app_name").unwrap();

    let path_to_command = matches.value_of("path").unwrap_or("/home/rt/projects/crm/docker");
    
    println!("{}", app_name);
    println!("{}", path_to_command);

    parse_logs(app_name, path_to_command)

}