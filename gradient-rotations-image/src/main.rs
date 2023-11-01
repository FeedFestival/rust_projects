use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub phones: Vec<String>,
}

fn main() {

    let name = "Daniel";
    let phone_nr = "123";
    let person = Person {
        age: 2,
        name: name.to_string(),
        phones: vec![phone_nr.to_string()],
    };

    let json_path = "data.json";
    gamescript::json_read_write::write(&person, json_path);

    let json_person: Person = gamescript::json_read_write::deserialize_json(json_path);

    println!("Deserialized JSON {:?}", json_person);

    // bytes

    let bin_path = "data.bin";
    gamescript::bin_read_write::write(&person, bin_path);

    let saved_person: Person = gamescript::bin_read_write::deserialize_bin(bin_path);

    println!("Deserialized BIN {:?}", saved_person);
}
