use std::{env, error::Error, ffi::OsStr, fs::File, io::BufReader, path::Path};

use rust_xlsxwriter::Workbook;
use unity_utage_book::{book::Book, reader::BookReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    let book = Book::from_reader(&mut BookReader::open(BufReader::new(File::open(
        args.get(1).unwrap(),
    )?))?)?;

    let filename = if let Some(name) = args.get(2) {
        name.clone()
    } else {
        let mut name = Path::new(&book.header.name)
            .file_name()
            .map(OsStr::to_str)
            .flatten()
            .map(ToString::to_string)
            .expect("Cannot read original file name from file");

        name.push_str(".xlsx");

        name
    };

    let mut workbook = Workbook::new();

    for sheet in book.sheets {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(&sheet.name[(sheet.name.rfind(":").unwrap() + 1)..])?;

        for (row_index, row) in sheet.rows.iter().enumerate() {
            for (col_index, col) in row.strings.iter().enumerate() {
                if col.is_empty() {
                    continue;
                }

                worksheet.write_string(row_index as _, col_index as _, col)?;
            }
        }
    }

    workbook.save(filename)?;
    Ok(())
}
