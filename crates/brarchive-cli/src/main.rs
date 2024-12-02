use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use clap::Parser;
use log::{error, info};
use crate::args::{CliArgs, CliSubcommand};
use crate::logger::setup_logger;

mod args;
mod logger;

fn main() {
    let args = CliArgs::parse();

    setup_logger(args.log_path);

    match args.command {
        CliSubcommand::Encode { path, out } => {
            let start_time = Instant::now();

            let out = out.unwrap_or(extract_file_name(&path).unwrap_or(PathBuf::from("brarchive")));
            let out = add_extension_if_missing(out, "brarchive");
            
            info!("Beginning to encode \"{}\" into \"{}\" archive", &path.display(), out.display());
            
            let read_dir = fs::read_dir(&path).unwrap_or_else(|err| {
                error!("Failed to read directory \"{}\": {}", path.display(), &err);
                exit(1);
            });
            
            let mut dir_entries = BTreeMap::new();
            
            for entry in read_dir {
                let entry = entry.unwrap_or_else(|err| {
                    error!("Failed to read directory entry of \"{}\": {}", path.display(), err);
                    exit(1);
                });
                
                let content = fs::read_to_string(&entry.path()).unwrap_or_else(|err| {
                    error!("Failed to read file \"{}\": {}", entry.path().display(), err);
                    exit(1);
                });
                
                let entry_name = entry.path();
                let entry_name = entry_name.strip_prefix(&path).unwrap_or_else(|err| {
                    error!("Failed to strip prefix from \"{}\": {}", entry.path().display(), err);
                    exit(1);
                });
                let entry_name = entry_name.to_str().unwrap_or_else(|| {
                    error!("Failed to convert file path to UTF-8: {}", entry.path().display());
                    exit(1);
                });
                
                dir_entries.insert(entry_name.to_string(), content);
            }
            
            let archive = brarchive::serialize(dir_entries).unwrap_or_else(|err| {
                error!("Failed to encode directory entries \"{}\": {}", err, path.display());
                exit(1);
            });
            
            fs::write(&out, &archive).unwrap_or_else(|err| {
                error!("Failed to write archive \"{}\": {}", out.display(), err);
            });

            info!("Successfully encoded archive in {}!", humantime::format_duration(start_time.elapsed()));
        }
        CliSubcommand::Decode { path, out } => {
            let start_time = Instant::now();

            let out = out.unwrap_or(extract_file_name(&path).unwrap_or(PathBuf::from("brarchive")));

            info!("Beginning to decode archive \"{}\" into \"{}\"", path.display(), out.display());

            let data = fs::read(&path).unwrap_or_else(|err| {
                error!("Failed to read archive file \"{}\": {}", path.display(), err);
                exit(1);
            });

            let archive = brarchive::deserialize(&data).unwrap_or_else(|err| {
                error!("Failed to decode archive file \"{}\": {}", path.display(), err);
                exit(1);
            });

            let exists = fs::exists(&out).unwrap_or_else(|err| {
                error!("Failed to check if output directory exists \"{}\": {}", path.display(), err);
                exit(1);
            });

            if exists && !out.is_dir() {
                error!("The output directory \"{}\" exist and is not a directory", path.display());
                exit(1);
            }

            let directory_contents = fs::read_dir(&out).ok();

            if let Some(contents) = directory_contents {
                if contents.count() > 0 || exists {
                    error!("The output directory \"{}\" already exists and is not empty", path.display());
                    exit(1);
                }
            }

            if !exists {
                fs::create_dir(&out).unwrap_or_else(|err| {
                    error!("Failed to create output directory \"{}\": {}", out.display(), err);
                    exit(1);
                });

                info!("Successfully created output directory \"{}\"", out.display());
            }

            for (file, contents) in archive {
                fs::write(out.join(&file), contents).unwrap_or_else(|err| {
                    error!("Failed to write output file \"{}\": {}", out.display(), err);
                });

                info!("Successfully decoded {:?}", &file);
            }

            info!("Successfully decoded archive in {}!", humantime::format_duration(start_time.elapsed()));
        }
    }
}


fn extract_file_name(path: &PathBuf) -> Option<PathBuf> {
    path.file_stem()
        .and_then(OsStr::to_str) // Convert OsStr to &str
        .map(PathBuf::from) // Convert &str to PathBuf
}

fn add_extension_if_missing(mut path: PathBuf, extension: &str) -> PathBuf {
    if path.extension().is_none() {
        path.set_extension(extension);
    }
    path
}
