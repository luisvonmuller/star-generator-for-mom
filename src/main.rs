use std::env;
use std::sync::{Arc, Mutex};
use std::{error::Error, fs::File, io::BufReader};
/* Some Stuff */
pub mod drawer;
pub mod helpers;

/* Pseudo Types */
pub type Students = Arc<Mutex<Vec<Student>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Student {
    pub name: String, /* Student's name like: João Pessoa */
    pub book: String, /* Books name like: Teens 3 */
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_information = File::open(format!("{}/sample.txt", env::current_dir()?.display()))?;
    let file_reader = BufReader::new(&file_information);
    let students_collections: Students = Arc::new(Mutex::new(Vec::new()));

    if let Ok(student_collection_populated) =
        helpers::students_collections(file_reader, students_collections)
    {
        process_stars(student_collection_populated);
    }

    Ok(())
}

pub fn process_stars(students_collection: Students) {
    if let Ok(data_collection) = students_collection.lock() {
        let _clone = data_collection.clone();
        for student in _clone {
            drawer::process(&student)
        }
    }
}
