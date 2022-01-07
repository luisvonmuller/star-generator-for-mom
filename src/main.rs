use std::iter::{IntoIterator, Iterator};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// The name should be "Our Wizards"

#[derive(Debug)]
struct Alumni {
    name: String,
    book: String,
}

impl IntoIterator for Alumni {
    type Item = Alumni;

    fn into_iter(&mut self) -> Self::IntoIter {
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html
        self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let alumnis = File::open("./sample.txt")?;
    let reader = BufReader::new(&alumnis);
    for alumni in reader.lines() {
        /* Multi Thread should begin here? */
        let alumni = alumni?.trim().to_owned();
        let alumni_info: Vec<Alumni> = alumni
            .split("-")
            .collect::<Vec<&str>>()
            .map(|position| Alumni {
                name: position.to_owned(),
                book: position.to_owned(),
            })
            .collect();
        println!("{:?}", alumni_info);
    }

    Ok(())
}
