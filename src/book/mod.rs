pub mod header;
pub mod sheet;

use std::io::{self, Read, Write};

use crate::{reader::BookReader, writer::BookWriter};

use self::{header::BookHeader, sheet::BookSheet};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Book {
    pub header: BookHeader,
    pub sheets: Vec<BookSheet>,
}

impl Book {
    pub fn from_reader(reader: &mut BookReader<impl Read>) -> io::Result<Self> {
        let mut sheets = Vec::with_capacity(reader.sheet_count() as _);
        for _ in 0..reader.sheet_count() {
            sheets.push(reader.read_next()?);
        }

        Ok(Book {
            header: reader.header().clone(),
            sheets,
        })
    }

    pub fn write_to_writer(&self, writer: impl Write) -> io::Result<()> {
        let mut writer = BookWriter::create(writer, &self.header, self.sheets.len() as _)?;

        for sheet in &self.sheets {
            writer.write_next(sheet)?;
        }

        Ok(())
    }
}
