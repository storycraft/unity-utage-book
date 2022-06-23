use std::{
    error::Error,
    io::{self, BufReader},
};

use unity_utage_book::{book::Book, reader::BookReader};

fn main() -> Result<(), Box<dyn Error>> {
    let book = Book::from_reader(&mut BookReader::open(BufReader::new(io::stdin()))?)?;
    serde_json::to_writer_pretty(io::stdout(), &book)?;
    Ok(())
}
