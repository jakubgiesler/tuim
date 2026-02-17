use std::{
    fs,
    process::Command,
};

use heapless::{
    String,
    Vec,
};

use crate::common;

pub fn run_man(on_file: &str) -> Option<std::string::String> {
    match Command::new("man").arg(on_file).output() {
        Ok(output) => Some(std::string::String::from_utf8_lossy(&output.stdout).to_string()),
        Err(_) => None,
    }
}

pub fn count_files_in_directory(dir: &str) -> usize {
    fs::read_dir(dir).map(|e| e.flatten().count()).unwrap_or(0)
}

pub fn search_in_directory(
    dir: &str,
    filter: Option<&str>,
    offset: usize,
) -> Vec<String<{ common::STRING_SIZE }>, { common::BUFFER_SIZE }> {
    let mut buffer = Vec::new();
    let mut c = 0;

    if let Ok(es) = fs::read_dir(dir) {
        for e in es.flatten() {
            if let Ok(file_name) = e.file_name().into_string() {
                if let Some(ref f) = filter {
                    if !file_name.contains(f) {
                        continue;
                    }
                }

                if c < offset {
                    c += 1;

                    continue;
                }

                if buffer.len() < buffer.capacity() {
                    let mut s: String<{ common::STRING_SIZE }> = String::new();

                    if s.push_str(&file_name).is_ok() {
                        buffer.push(s).ok();
                    }
                } else {
                    break;
                }
            }
        }
    }

    buffer
}
