use std::{error::Error, fs::File, io::BufReader};
use tokio_stream::{self as stream, StreamExt};

pub mod drawer;
pub mod helpers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Student {
    pub name: String, /* Student's name like: João Pessoa */
    pub book: String, /* Books name like: Teens 3 */
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_information = File::open("./sample.txt")?;
    let file_reader = BufReader::new(&file_information);
    let mut students_collection: Vec<Student> = Vec::new(); // Initializing the colection.
                                                            /* Async Structuring the Collection with Tokyo Stream */

    /* Ok Then here I'll have to Use and Arc because Students_collection must be borrowed
    with some 'static life time, and to be shared between threads in a safe way... */
    let mut students_collection = stream::iter(helpers::students_collections(
        file_reader,
        &mut students_collection,
    )?);

    /* I think this will block thread spawing ? */
    while let Some(student) = students_collection.next().await {
        tokio::spawn(async {
            println!("Spawning Thread...");
            drawer::process(student).await;
        });

        return Ok(());
    }

    Ok(())
}
