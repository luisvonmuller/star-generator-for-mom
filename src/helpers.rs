use crate::{Student, Students};
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn students_collections(
    reader: BufReader<&File>,
    students_collection: Students,
) -> Result<Students, Box<dyn Error>> {
    for student in reader.lines() {
        let student = student?.trim().to_owned();
        let student = student.split('-').collect::<Vec<&str>>();

        /* Change to mutex */
        if let Ok(mut col) = students_collection.lock() {
            col.push(Student {
                name: student[0].to_owned(),
                book: student[1].to_owned(),
            });
        }
    }

    Ok(students_collection)
}
