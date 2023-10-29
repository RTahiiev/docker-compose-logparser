use clap::{App, Arg};
use crate::parser::parse_logs;
use crate::config::get_config;


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
        .arg(Arg::with_name("container_name")
            .short("c")
            .long("container")
            .value_name("container")
            .help("Container name")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .value_name("/path/where/execute")
            .help("Path where command must be executed")
            .required(false)
            .takes_value(true))
        .get_matches();

    let config = get_config();
    let app_name = matches.value_of("app_name").unwrap();
    let container_name = matches.value_of("container_name");

    let default_path = config.default_path(app_name);
    let path_to_command = matches.value_of("path").unwrap_or(&default_path);
    
    println!("{}", app_name);
    println!("{}", path_to_command);

    parse_logs(container_name, path_to_command)

}