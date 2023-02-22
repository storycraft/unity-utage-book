use std::{
    error::Error,
    io::{self, BufWriter},
};

use unity_utage_book::book::Book;

fn main() -> Result<(), Box<dyn Error>> {
    let book: Book = serde_json::from_reader(io::stdin())?;

    book.write_to_writer(&mut BufWriter::new(io::stdout()))?;
    Ok(())
}
