use crate::file_read_write;
use bincode::{deserialize, serialize};

pub fn write<T: serde::Serialize>(target: &T, path: &str) {
    let encoded: Vec<u8> = serialize(target).unwrap();
    file_read_write::write_bytes(encoded, path);
}

pub fn deserialize_bin<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let data = file_read_write::read_bytes(path);
    let decoded: T = deserialize(&data).unwrap();
    decoded
}
