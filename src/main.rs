use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::io::Write;


pub struct PersonRecord {
    name: String,
    age: u8,
    phones: String,
}

impl PersonRecord {
    pub fn new(name: String, age: u8, phones: String) -> Self {
        PersonRecord { name, age, phones }
    }

}


impl Display for PersonRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{},{},{}", self.name, self.age, self.phones)
    }
}

impl Serialize for PersonRecord{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer,
        {
           let mut state =  serializer.serialize_struct("PersonRecord", 3)?;
            state.serialize_field("name", &self.name)?;
            state.serialize_field("age", &self.age)?;
            state.serialize_field("phones", &self.phones)?;
            state.end()

        }

}


pub fn person_to_json(person_param : PersonRecord) -> Result<String, serde_json::Error>{
    let person_json = serde_json::to_string(&person_param).unwrap();

    Ok(person_json)

}

fn main() -> std::io::Result<()> {
    let mut person: Vec<PersonRecord> = Vec::new();
    let mut file_lines: Vec<String> = Vec::new();

    let p1 = PersonRecord::new(String::from("Justin"), 18, String::from("867-5309"));

    person.insert(0, p1);

    let  p2 = PersonRecord::new(String::from("Jocelyn"), 24, String::from("555-5555"));

    person.insert(1, p2);

    let mut index = 0;
    for x in person {
        let as_json = person_to_json(x);
        let copy_json = as_json.unwrap();
        println!("{}", copy_json);
        file_lines.insert(index, copy_json);
        index += 1
    }

    let mut output_json = std::fs::File::create("test.json")?;
    for f in file_lines{
        let line = f.to_string() + "\n";
        output_json.write_all(line.as_bytes());
    }


    Ok(())
}
