use std::{env, io, process};
use std::fs::{File, OpenOptions};
use std::path::Path;
use filetime::{set_file_times, FileTime};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() - 1 != 1 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file_name = &args[1];
    let file_path = Path::new(file_name);

    let _file = if file_path.exists() {
        OpenOptions::new().read(true).open(file_path)?
    } else {
        File::create(&file_path)?
    };

    let now = FileTime::now();

    set_file_times(&file_path, now, now)?;

    Ok(())
}
