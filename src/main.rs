/*
*  fc main
*  SPDX-License-Identifier: MIT
*  Copyright (C) 2021 Benjamin Schilling
*/

// Rust Standard Library
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::{ffi::OsString, io};
// CLI
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("fc")
        .version("0.2.0")
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
        .arg(
            Arg::with_name("confirm")
                .short("c")
                .takes_value(false)
                .help("Immediately confirm deletion.")
                .required(false),
        )
        .get_matches();

    // auto confirm?
    let auto_confirm = matches.is_present("confirm");

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
    println!("Going to delete the following files:");
    for filename in &filenames {
        if filenames.iter().filter(|&n| n == filename).count() < 2 {
            files_to_delete.push(filename);
            println!("{:?}", filename);
        }
    }
    if files_to_delete.len() == 0 {
        println!("No files matching extensions found.");
        return;
    }
    if !auto_confirm {
        println!("Confirm (y/Y):");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer = buffer.trim().to_string();
        if buffer == "y" || buffer == "Y" {
            delete(files_to_delete, extensions, file_path);
            print!("Deleted files.")
        }
    } else {
        delete(files_to_delete, extensions, file_path);
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

fn delete(files_to_delete: Vec<&OsString>, extensions: clap::Values, file_path: &str) {
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

#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;
    use fs::File;

    #[test]
    fn good_case() {
        // create directory
        fs::create_dir("./test_data/").unwrap();
        // create test files, one will be cleaned, the other stays
        let _file = File::create("./test_data/foo.jpg").unwrap();
        let _file = File::create("./test_data/bar.jpg").unwrap();
        let _file = File::create("./test_data/bar.orf").unwrap();
        print!("cleaning test_now");
        // run fc for test directory with "alt" and "test" file ending
        let mut fs_process = Command::new("target/release/fc")
            .args(&["-p", "test_data", "-e", "jpg", "-e", "orf", "-c"])
            .spawn()
            .expect("Failed to start fc process.");
        let _ = fs_process.wait();

        let file_should_not_exist = match fs::metadata("./test_data/foo.jpg") {
            Ok(o) => o.is_file(),
            _ => false,
        };
        let file_shout_exist = fs::metadata("./test_data/bar.jpg").unwrap().is_file();

        let _ = fs::remove_dir_all("./test_data/");

        assert!(file_shout_exist && !file_should_not_exist);
    }
}
