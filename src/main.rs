use std::sync::Arc;
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

    /* Ok Then here I'll have to Use and Arc because Students_collection must be borrowed
    with some 'static life time, and to be shared between threads in a safe way...

        https://doc.rust-lang.org/std/sync/struct.Arc.html
        https://stackoverflow.com/questions/68059435/argument-requires-that-value-is-borrowed-for-static-not-working-for-non-copy

    I'think Arc Is enought because...

    The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. I
    nvoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc,
    while increasing a reference count. When the last Arc pointer to a given allocation is destroyed,
    the value stored in that allocation (often referred to as “inner value”) is also dropped.
    Shared references in Rust disallow mutation by default, and Arc is no exception: you cannot generally obtain a mutable reference
    to something inside an Arc. If you need to mutate through an Arc, use Mutex, RwLock, or one of the Atomic types.
    */
    let mut students_collection = stream::iter(helpers::students_collections(file_reader)?);
    /*!I think the arc must be created here within a mutex, by that I mean giving main threads ownership to later itarete overself.think */
    /* I think this will block thread spawing ? */
    while let Some(student) = students_collection.next().await {
        tokio::spawn(async move {
            println!("Spawning Thread...");
            drawer::process(&student).await;
        });

        return Ok(());
    }

    Ok(())
}
