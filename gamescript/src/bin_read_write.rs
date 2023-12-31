use crate::file_read_write;
use bincode::{deserialize, serialize};

pub fn write<T: serde::Serialize>(target: &T, path: &str) {
    let encoded: Vec<u8> = serialize(target).unwrap();
    file_read_write::write_bytes(encoded, path);
}

pub fn deserialize_bin<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let data = file_read_write::read_bytes(path);
    let deserialized: Result<T, Box<bincode::ErrorKind>> = deserialize(&data);
    let decoded: Option<T>;
    match deserialized {
        Ok(des) => {
            decoded = Some(des);
        },
        Err(_) => {
            println!("Could not deserialize, is it possible that you changed the Type? Consider deleting the bin file and recreating it.");
            decoded = None;
        },
    }
    decoded.unwrap()
}
