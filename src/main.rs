use std::{time::Instant, io::{Write, self}, error::Error};
use clap::Parser;

mod formatter;
mod analyser;

// Command line
#[derive(Parser)]
struct Cli {
    /// The path of the directory to read
    path: String,

    /// Number of files to display
    #[clap(default_value_t = 10)]
    nb_files: usize
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let dir_name = &args.path;

    let now = Instant::now();
    println!("");

    println!(" ==> Analysing {} ", dir_name);
    io::stdout().flush()?;

    let files = analyser::analyse_dir(dir_name)?;

    let nb_files = files.len();
    let total_size = files.iter().fold(0, |acc, x| acc + x.file_size);
    let elapsed_time = now.elapsed();
    if nb_files == 0 {
        println!("     No files");
        return Ok(());
    }
    println!(
        "     {} files with a total size of {} ({})",
        nb_files,
        formatter::format_size(total_size),
        formatter::format_time(elapsed_time.as_millis())
    );
    println!("");
    let nb_files = std::cmp::min(files.len(), args.nb_files);
    for n in 0..nb_files {
        println!(
            " {0: <40} {1: <10} {2: <10}",
            files[n].file_name,
            formatter::format_size(files[n].file_size),
            files[n].file_dir
        );
    }
    println!("");
    Ok(())
}

fn main() -> Result<(), String> {
    run().map_err(|e| e.to_string())?;

    Ok(())
}
