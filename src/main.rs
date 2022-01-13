use std::env;
use std::sync::{Arc, Mutex};
use std::{error::Error, fs::File, io::BufReader};

pub mod drawer;
pub mod helpers;

/* Type Aliasing just to not write this allways, actually not used anymore, I've removed the thread Pool. */
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
    /* Will Rearange the data collection for us as well trim things */
    if let Ok(student_collection_populated) =
        helpers::students_collections(file_reader, students_collections)
    {
        process_stars(student_collection_populated);
    }
    /* This is not that necessary anymore, but I'll let it here because I'm lazy and this is just for my mom. */
    Ok(())
}

/* Calls Drawer on the mutex guard. */
pub fn process_stars(students_collection: Students) {
    if let Ok(data_collection) = students_collection.lock() {
        let _clone = data_collection.clone();
        for student in _clone {
            drawer::process(&student)
        }
    }
}
