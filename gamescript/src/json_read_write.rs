use crate::file_read_write;

pub fn write<T: serde::Serialize>(target: &T, path: &str) {
    let json_string = serde_json::to_string(&target).unwrap();
    file_read_write::write_text(json_string, path);
}

pub fn deserialize_json<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let json_string = file_read_write::read_text(path);
    let person_from_json: T = serde_json::from_str(json_string.as_str()).unwrap();
    person_from_json
}
