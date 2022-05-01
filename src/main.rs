mod convert;
use convert::convert;

use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let mut args = env::args();
    let own_path = args.next().unwrap();
    let target_file = args.next();
    match target_file {
        Some(file) => convert_file(file),
        None => convert_all_files(own_path),
    }
}

fn convert_file(path: impl AsRef<Path>) {
    let content = fs::read_to_string(path.as_ref()).expect("failed to read file");
    let out = convert(&content);
    let out_name = format!("{}_converted.{}",
        path.as_ref().file_stem().expect("invalid file name").to_str().expect("invalid file name"),
        path.as_ref().extension().expect("invalid file extension").to_str().expect("invalid file extension")
    );
    let out_path = path.as_ref().with_file_name(out_name);
    fs::write(out_path, out).expect("failed to write file");
}

fn convert_all_files(own_path: impl AsRef<Path>) {
    eprintln!("{}", own_path.as_ref().display());
    let directory = own_path.as_ref().parent().expect("failed to read directory");
    for header in headers_in_directory(directory) {
        convert_file(header);
    }
}

fn headers_in_directory(directory: &Path) -> Vec<PathBuf> {
    fs::read_dir(directory).expect("failed to read directory")
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if let Some("wotwrh") = extension.to_str() {
                        return Some(path);
                    }
                }
            }
            None
        })
        .collect()
}
