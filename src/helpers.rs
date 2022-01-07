use crate::Student;
use std::sync::Arc;
use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn students_collections(reader: BufReader<&File>) -> Result<Arc<Vec<Student>>, Box<dyn Error>> {
    let mut students_collection = Vec::new();
    for student in reader.lines() {
        let student = student?.trim().to_owned();
        let _student = student.split('-').collect::<Vec<&str>>();

        students_collection.push(Student {
            name: _student[0].to_owned(),
            book: _student[1].to_owned(),
        });
    }

    Ok(Arc::new(students_collection))
}
