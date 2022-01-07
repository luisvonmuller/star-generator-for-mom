use std::{error::Error, fs::File, io::BufReader};

pub mod drawer;
pub mod helpers;

// The name should be "Our Wizards"

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Student {
    pub name: String, /* Student's name like: João Pessoa */
    pub book: String, /* Books name like: Teens 3 */
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_information = File::open("./sample.txt")?;
    let file_reader = BufReader::new(&file_information);
    let mut students_collection: Vec<Student> = Vec::new(); // Initializing the colection.
    let students_collection = helpers::students_collections(file_reader, &mut students_collection)?;
    println!("{:?}", students_collection);
    Ok(())
}
