use std::fs::{self, File};
use std::io::{self, BufRead, Write};

fn write_log(path: &str, entries: &[&str]) -> io::Result<()> {
    let mut file = File::create(path)?;

    for entry in entries {
        writeln!(file, "{}", entry)?;
    }

    Ok(())
}

fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().count())
}

// TODO 4:write a function that recursively lists all .rs files
// under a given directory using std
fn list_rs_files(dir: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            list_rs_files(path.to_str().unwrap())?;
        } else if let Some(ext) = path.extension() {
            if ext == "rs" {
                println!("{}", path.display());
            }
        } else if let Some(ext) = path.extension()
            && ext == "rs"
        {
            println!("{}", path.display());
        }
    }

    Ok(())
}

pub fn main() -> io::Result<()> {
    let path = "output.log";

    let entries = vec![
        "INFO Server started",
        "WARN High memory usage",
        "ERROR Disk full",
        "INFO Backup complete",
    ];

    write_log(path, &entries)?;

    let n = count_lines(path)?;
    println!("Wrote {} lines to {}", n, path);

    // Read file
    let content = fs::read_to_string(path)?;

    let errors: Vec<&str> = content
        .lines()
        .filter(|line| line.starts_with("ERROR"))
        .collect();

    println!("Error lines: {:?}", errors);

    println!("\nRust files in current directory:");
    list_rs_files(".")?;

    fs::remove_file(path)?;

    Ok(())
}
