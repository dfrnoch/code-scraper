use clap::Parser;
use env_logger::Builder;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use log::{debug, error, info, warn, LevelFilter};
use std::fs::{read, read_dir, File};
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source folder path
    source_folder: String,

    /// Custom ignore file path
    #[arg(default_value = ".gitignore")]
    custom_ignore_file: String,

    /// Enable logging
    #[arg(long)]
    log: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    if args.log {
        initialize_logging()?;
    }

    let ignore_patterns = read_ignore_patterns(&args.source_folder, &args.custom_ignore_file)?;

    info!("Creating output file: output.txt");
    let mut output = BufWriter::new(
        File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open("output.txt")?,
    );

    info!("Processing files in: {}", args.source_folder);
    if let Err(e) = process_files(
        &PathBuf::from(&args.source_folder),
        &ignore_patterns,
        &mut output,
        &PathBuf::from(&args.source_folder),
    ) {
        error!("Error processing files: {}", e);
        return Err(e);
    }

    info!("Processing completed successfully.");
    Ok(())
}

fn initialize_logging() -> Result<(), Error> {
    let log_file = "data.log";
    let mut builder = Builder::new();
    builder
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, LevelFilter::Debug)
        .target(env_logger::Target::Pipe(Box::new(File::create(log_file)?)));
    builder.init();
    Ok(())
}

fn should_ignore(path: &Path, gitignore: &Gitignore) -> bool {
    gitignore.matched(path, path.is_dir()).is_ignore()
}

fn read_ignore_patterns(source_folder: &str, ignore_file: &str) -> Result<Gitignore, Error> {
    let ignore_path = if ignore_file == ".gitignore" {
        let mut path = PathBuf::from(source_folder);
        path.push(ignore_file);
        path
    } else {
        PathBuf::from(ignore_file)
    };

    let mut builder = GitignoreBuilder::new(source_folder);

    // Always ignore the .git directory
    builder
        .add_line(None, ".git/")
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    if ignore_path.exists() {
        info!("Reading ignore file: {:?}", ignore_path);
        builder.add(&ignore_path);
    } else {
        warn!("No ignore file found at: {:?}", ignore_path);
    };
    builder.build().map_err(|e| Error::new(ErrorKind::Other, e))
}

fn process_files(
    folder: &Path,
    ignore_patterns: &Gitignore,
    output: &mut BufWriter<File>,
    base_dir: &Path,
) -> Result<(), Error> {
    debug!("Processing folder: {:?}", folder);

    for entry in read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if should_ignore(&path, &ignore_patterns) {
            debug!("Ignoring path: {:?}", path);
            continue;
        }

        if path.is_file() {
            if let Some(_) = path.extension() {
                let relative_path = path.strip_prefix(base_dir).unwrap();
                debug!("Processing file: {}", relative_path.display());

                let file_content = read(&path)?;

                if let Ok(content) = std::str::from_utf8(&file_content) {
                    write!(output, "----- {} -----\n", relative_path.display())?;
                    output.write_all(content.as_bytes())?;
                    output.write_all(b"\n\n")?; // Add a blank line between file contents
                } else {
                    warn!("Skipping non-UTF-8 file: {}", relative_path.display());
                }
            } else {
                debug!("Skipping file without extension: {:?}", path);
            }
        } else if path.is_dir() {
            debug!("Recursively processing directory: {:?}", path);
            process_files(&path, ignore_patterns, output, base_dir)?;
        }
    }

    Ok(())
}
