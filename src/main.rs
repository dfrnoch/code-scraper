use env_logger::Builder;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use log::LevelFilter;
use std::fs::{read, read_dir, File};
use std::io::{Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    let log_file = "data.log";
    let mut builder = Builder::new();
    builder
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter(None, LevelFilter::Debug)
        .target(env_logger::Target::Pipe(Box::new(
            File::create(log_file).unwrap(),
        )));
    builder.init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        log::error!("Invalid arguments. Usage: <source_folder> <output_file>");
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Usage: <source_folder> <output_file>",
        ));
    }

    let source_folder = &args[1];
    let output_file = &args[2];

    log::info!("Reading .gitignore patterns from: {}", source_folder);
    let ignore_patterns = read_gitignore_patterns(source_folder).unwrap();

    log::info!("Creating output file: {}", output_file);
    let mut output = File::create(output_file)?;

    log::info!("Processing files in: {}", source_folder);
    process_files(
        &PathBuf::from(source_folder),
        &ignore_patterns,
        &mut output,
        &PathBuf::from(source_folder),
    )?;

    log::info!("Processing completed successfully.");
    Ok(())
}

fn should_ignore(path: &Path, gitignore: &Gitignore) -> bool {
    gitignore.matched(path, path.is_dir()).is_ignore()
}

fn read_gitignore_patterns(source_folder: &str) -> Option<Gitignore> {
    let gitignore_path = Path::new(source_folder).join(".signore");
    let mut builder = GitignoreBuilder::new(source_folder);

    if gitignore_path.exists() {
        log::info!("Reading .gitignore file: {:?}", gitignore_path);
        builder.add(&gitignore_path);
    } else {
        log::warn!("No .gitignore file found in: {:?}", source_folder);
    };
    builder.build().ok()
}

fn process_files(
    folder: &Path,
    ignore_patterns: &Gitignore,
    output: &mut File,
    base_dir: &Path,
) -> Result<(), Error> {
    log::debug!("Processing folder: {:?}", folder);

    for entry in read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if should_ignore(&path, &ignore_patterns) {
            log::debug!("Ignoring path: {:?}", path);
            continue;
        }

        if path.is_file() {
            if let Some(_extension) = path.extension() {
                let relative_path = path.strip_prefix(base_dir).unwrap();
                log::debug!("Processing file: {}", relative_path.display());

                let file_content = read(&path)?;

                if let Ok(content) = std::str::from_utf8(&file_content) {
                    writeln!(output, "----- {} -----", relative_path.display())?;
                    writeln!(output, "{}", content)?;
                    writeln!(output)?; // Add a blank line between file contents
                } else {
                    log::warn!("Skipping non-UTF-8 file: {}", relative_path.display());
                }
            } else {
                log::debug!("Skipping file without extension: {:?}", path);
            }
        } else if path.is_dir() {
            log::debug!("Recursively processing directory: {:?}", path);
            process_files(&path, ignore_patterns, output, base_dir)?;
        }
    }

    Ok(())
}
