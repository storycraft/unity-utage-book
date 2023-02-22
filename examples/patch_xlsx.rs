use std::{
    collections::HashMap,
    env,
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use calamine::{Reader, Xlsx};
use unity_utage_book::{book::Book, reader::BookReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    let mut book = Book::from_reader(&mut BookReader::open(BufReader::new(File::open(
        args.get(1).unwrap(),
    )?))?)?;

    let mut workbook = Xlsx::new(BufReader::new(File::open(args.get(2).unwrap())?))?;

    let filename = if let Some(name) = args.get(3) {
        name.clone()
    } else {
        Path::new(&book.header.name)
            .file_name()
            .map(OsStr::to_str)
            .flatten()
            .map(ToString::to_string)
            .expect("Cannot read original file name from file")
    };

    let mut map = HashMap::new();
    for sheet in &mut book.sheets {
        map.insert(
            sheet.name[(sheet.name.rfind(":").unwrap() + 1)..].to_string(),
            sheet,
        );
    }

    for (sheet_name, sheet) in workbook.worksheets() {
        let book_sheet = map.get_mut(&sheet_name).unwrap();

        for (row_index, row) in sheet.rows().enumerate() {
            let strings = &mut book_sheet.rows.get_mut(row_index).unwrap().strings;

            for (col_index, col) in row.iter().enumerate() {
                let col_string = col.to_string();
                if !col_string.is_empty() {
                    *strings.get_mut(col_index).unwrap() = col_string;
                }
            }
        }
    }

    book.write_to_writer(BufWriter::new(File::create(filename)?))?;

    Ok(())
}
