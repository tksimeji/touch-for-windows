use std::{env, io, process};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use filetime::{set_file_times, FileTime};

struct CommandOption {
    name: &'static str,
    has_value: bool,
}

impl CommandOption {
    const OPTIONS: &'static [CommandOption] = &[
        CommandOption {
            name: "c",
            has_value: false,
        }
    ];

    fn alias(&self) -> String {
        format!("-{}", self.name.chars().next().unwrap())
    }

    fn find_option(arg: &str) -> Option<&'static CommandOption> {
        if arg.starts_with("--") {
            CommandOption::OPTIONS.iter().find(|o| o.name == &arg[2..])
        } else if arg.starts_with('-') {
            CommandOption::OPTIONS.iter().find(|o| o.alias() == arg)
        } else {
            None
        }
    }
}

fn main() -> io::Result<()> {
    let command_line_args: Vec<String> = env::args().collect();

    let mut options = HashMap::new();
    let mut args = Vec::new();

    let mut i = 1;

    while i < command_line_args.len() {
        let arg = &command_line_args[i];

        if let Some(option) = CommandOption::find_option(arg) {
            if option.has_value {
                if i + 1 < command_line_args.len() {
                    options.insert(option.name.to_string(), command_line_args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: Option '{}' requires a value", option.name);
                }
            } else {
                options.insert(option.name.to_string(), String::from("true"));
            }
        } else {
            args.push(arg.clone());
        }

        i += 1;
    }

    if args.len() != 1 {
        eprintln!("Usage: {} <file>", command_line_args[0]);
        process::exit(1);
    }

    let file_name = &args[0];
    let file_path = Path::new(file_name);

    if ! file_path.exists() {
        if options.contains_key("c") {
            process::exit(0);
        }

        File::create(file_path)?;
    }

    let now = FileTime::now();

    set_file_times(&file_path, now, now)?;

    Ok(())
}
