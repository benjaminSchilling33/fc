use std::ffi::OsString;
use std::path::Path;
// Rust Standard Library
use std::fs;
use std::fs::DirEntry;
// CLI
extern crate clap;
use clap::{App, Arg};
fn main() {
    let matches = App::new("fc")
        .version("0.1.0")
        .author("Benjamin Schilling <benjamin.schilling33@gmail.com>")
        .about("Among all files with the given file extensions, remove individual files.")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("Path to the directory for clean up.")
                .required(true),
        )
        .arg(
            Arg::with_name("extensions")
                .short("e")
                .long("extension")
                .takes_value(true)
                .help("File extension in scope for clean-up.")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    // read specified file extensions in scope of checking
    let extensions = matches.values_of("extensions").unwrap();

    // create list of files in directory with the given extensions
    let file_path = matches.value_of("path").unwrap();

    let files = fs::read_dir(file_path).unwrap();
    let filtered_files: Vec<_> = files
        .filter_map(Result::ok)
        .filter(|d| match_extension(d, extensions.clone()))
        .collect();

    // extract all file names while ignoring file extension
    let filenames: Vec<OsString> = filtered_files
        .into_iter()
        .map(|f| f.path().to_owned()) // take the path and take ownership
        .map(|f| f.file_stem().unwrap().to_owned()) // take the filestem and take ownership
        .collect();

    let mut files_to_delete = Vec::new();

    for filename in &filenames {
        if filenames.iter().filter(|&n| n == filename).count() < 2 {
            files_to_delete.push(filename);
        }
    }
    // delete files
    for file in files_to_delete {
        for ext in extensions.clone() {
            let filename =
                format!("{}\\{:?}.{}", file_path, file.to_str().unwrap(), ext).replace("\"", "");
            if Path::new(&filename).exists() {
                match std::fs::remove_file(format!("{}", filename)) {
                    Ok(_o) => {
                        println!("Removed file: {}", filename);
                    }
                    Err(_e) => {
                        eprintln!("Could not remove file: {}", filename);
                    }
                }
            }
        }
    }
}

fn match_extension(file: &DirEntry, extensions: clap::Values) -> bool {
    for ext in extensions {
        if let Some(e) = file.path().extension() {
            if e.eq_ignore_ascii_case(ext) {
                return true;
            };
        }
    }
    return false;
}
