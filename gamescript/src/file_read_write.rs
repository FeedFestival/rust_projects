use std::{
    fs::{File, metadata},
    io::{Read, Write}, env,
};

pub fn dir_name(lib_name: &str) -> Option<String> {
    if let Ok(project_dir) = env::current_dir() {
        let mut project_dir = project_dir.to_string_lossy().to_string();
        project_dir = project_dir.replace(lib_name, "");
        return Some(project_dir);
    }
    return None;
}

pub fn write_text(string: String, path: &str) {
    let mut file = open_or_create(path);
    file.write_all(&string.as_bytes());
}

pub fn read_text(path: &str) -> String {
    let mut file = open_file(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string);

    string
}

pub fn write_bytes(encoded: Vec<u8>, path: &str) {
    let mut file = open_or_create(path);
    file.write_all(&encoded);
}

pub fn read_bytes(path: &str) -> Vec<u8> {
    let mut file = open_or_create(path);
    let mut data: Vec<u8> = vec![];
    file.read_to_end(&mut data);
    return data;
}

fn open_or_create(path: &str)-> File {
    // Check if the file already exists
    if metadata(path).is_ok() {
        let file_result = open_file(path);
        match file_result {
            Some(f) => {
                return f;
            },
            None => return create_file(path).unwrap(),
        }
    }

    create_file(path).unwrap()
}

fn create_file(path: &str) -> Option<File> {
    let file_result: Result<File, std::io::Error> = File::create(path);
    match file_result {
        Ok(f) => {
            return Some(f);
        }
        Err(err) => {
            // Handle the error if the file couldn't be created
            eprintln!("Error creating the file file: {}", err);
            return None;
        }
    }
}

fn open_file(path: &str) -> Option<File> {
    let file_bin_result = File::open(path);
    match file_bin_result {
        Ok(f) => {
            Some(f)
        }
        Err(err) => {
            // Handle the error if the file couldn't be opened
            eprintln!("Error opening the file file: {}", err);
            None
        },
    }
}
