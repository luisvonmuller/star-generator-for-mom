use crate::Student;
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn students_collections<'a>(
    reader: BufReader<&File>,
    students_collection: &'a mut Vec<Student>,
) -> Result<&'a mut Vec<Student>, Box<dyn Error>> {
    for student in reader.lines() {
        let student = student?.trim().to_owned();
        let _student = student.split('-').collect::<Vec<&str>>();

        students_collection.push(Student {
            name: _student[0].to_owned(),
            book: _student[1].to_owned(),
        });
    }

    Ok(students_collection)
}
