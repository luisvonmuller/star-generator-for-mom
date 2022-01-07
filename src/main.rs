extern crate jemallocator;
extern crate num_cpus;
extern crate threadpool;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::sync::{Arc, Mutex};
use std::{error::Error, fs::File, io::BufReader};
// use tokio_stream::{self as stream, StreamExt};

pub mod drawer;
pub mod helpers;

/* Pseudo Types */
pub type Students = Arc<Mutex<Vec<Student>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Student {
    pub name: String, /* Student's name like: João Pessoa */
    pub book: String, /* Books name like: Teens 3 */
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_information = File::open("./sample.txt")?;
    let file_reader = BufReader::new(&file_information);

    let students_collections: Students = Arc::new(Mutex::new(Vec::new()));
    if let Ok(student_collection_populated) =
        helpers::students_collections(file_reader, students_collections)
    {
        process_stars(student_collection_populated).await;
    }

    Ok(())
}

pub async fn process_stars(students_collection: Students) {
    use threadpool::ThreadPool;

    if let Ok(data_collection) = students_collection.lock() {
        let logical_cores_number = num_cpus::get() * 2; // Will get Logical Cores ? - I don't know if its valid for every...
        let our_thread_pool = ThreadPool::new(logical_cores_number);
        // !IM DONE FOR THE MORNING.
        for student in &mut data_collection {
            // Todo - Do stuff with the mutex  now, like passing it to every funcking job. But the vector... anoys.
            our_thread_pool.execute(move || println!("Timbum 😎"));
        }
    };

    println!("So far, so Great!");
}
