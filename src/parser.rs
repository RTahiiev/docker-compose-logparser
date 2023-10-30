use serde_json::Value;
use std::process::{Command, Stdio};
use std::io::{self, BufRead};


pub fn parse_logs(container_name: Option<&str>, current_path: &str) -> Result<(), std::io::Error> {
    let mut command_args = vec!["compose", "logs", "-f"];
    if let Some(container) = container_name {
        command_args.push(container)
    };
    let child_process = Command::new("docker")
        .args(command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .current_dir(current_path)
        .spawn();

    let find_field = "message";
    let time_field = "time";
    let pathname_field = "pathname";
    let lineno_field = "lineno";
    let level_field = "levelname";

    if let Ok(mut child) = child_process {
        // Create readers for stdout and stderr
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let stderr = child.stderr.take().expect("Failed to open stderr");

        // Create BufRead instances for reading stdout and stderr
        let stdout_reader = io::BufReader::new(stdout);
        let stderr_reader = io::BufReader::new(stderr);

        // Process stdout and stderr streams
        for line in stdout_reader.lines() {
            match line {
                Ok(line) => {
                    match line.find('{') {
                        Some(json_start) => {
                            let json_str = &line[json_start..];
                            if let Ok(json_value) = serde_json::from_str::<Value>(json_str) {
                                // Assuming the JSON is an object, you can extract it as a dictionary
                                if let Some(json_dict) = json_value.as_object() {
                                    let mut unescaped_message: String = String::new();
                                    let mut unescaped_time: String = String::new();
                                    let mut pathname: String = String::new();
                                    let mut lineno: String = String::new();
                                    let mut level: String = String::new();
                                    if let Some(value) = json_dict.get(find_field) {
                                        unescaped_message = serde_json::from_str(&value.to_string()).unwrap();
                                    }
                                    if let Some(value) = json_dict.get(time_field) {
                                        unescaped_time = serde_json::from_str(&value.to_string()).unwrap();
                                    }
                                    if let Some(value) = json_dict.get(pathname_field) {
                                        pathname = value.to_string();
                                    }
                                    if let Some(value) = json_dict.get(level_field) {
                                        level = value.to_string();
                                    }
                                    if let Some(value) = json_dict.get(lineno_field) {
                                        lineno = value.to_string();
                                    }

                                    println!("{} {} [{}:{}]: {}", unescaped_time, level, pathname, lineno, unescaped_message);
                                }
                            }
                        },
                        None => println!("{}", line),
                    }
                }
                Err(e) => eprintln!("stderr: {}", e)
            }
        }

        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                eprintln!("stderr: {}", line);
            }
        }
        
        let status = child.wait()?;
        println!("Child process exited with: {:?}", status);
    } else {
        eprintln!("Failed to start the child process.");
    }

    Ok(())
}
