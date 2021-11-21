extern crate cc;

use std::fs::read_dir;
use std::path::Path;

fn main() {
    let qrencode_dir = Path::new("libqrencode-external");

    // Get all .cpp files from external libqrencode
    let qrencode_cpp_files = read_dir(qrencode_dir).unwrap().filter_map(|e| {
        let path = e.unwrap().path();
        let file = path.file_name().unwrap().to_str().unwrap();
        if file.ends_with("cpp") {
            Some(path)
        } else {
            None
        }
    });

    cc::Build::new()
        .files(qrencode_cpp_files)
        .file("src/qrencode_wrapper.cpp")
        .include(qrencode_dir)
        .include(qrencode_dir.join("include"))
        .cpp(true)
        .pic(true) // Position Independent Code.
        .shared_flag(true)
        .compile("qrencode");
}