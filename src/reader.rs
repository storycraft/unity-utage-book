use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{self, Read};

use crate::book::{
    header::BookHeader,
    sheet::{BookSheet, BookSheetRow},
};

#[derive(Debug)]
pub struct BookReader<R> {
    reader: R,
    header: BookHeader,
    sheet_count: u32,
}

impl<R: Read> BookReader<R> {
    pub fn open(mut reader: R) -> io::Result<Self> {
        let header = BookHeader {
            object_flag: reader.read_u32::<LittleEndian>()?,
            parent_object: reader.read_u32::<LittleEndian>()?,
            internal: reader.read_u32::<LittleEndian>()?,
            game_object: reader.read_u32::<LittleEndian>()?,
            enabled: reader.read_u32::<LittleEndian>()? == 1,
            editor_hide_flag: reader.read_u32::<LittleEndian>()?,
            script: reader.read_u32::<LittleEndian>()?,
            name: read_string(&mut reader)?,
            import_version: reader.read_u32::<LittleEndian>()?,
        };

        let sheet_count = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            reader,
            header,
            sheet_count,
        })
    }

    pub fn header(&self) -> &BookHeader {
        &self.header
    }

    pub fn sheet_count(&self) -> u32 {
        self.sheet_count
    }

    fn read_next_row(&mut self) -> io::Result<BookSheetRow> {
        let row_index = self.reader.read_u32::<LittleEndian>()?;

        let strings = {
            let strings_len = self.reader.read_u32::<LittleEndian>()?;
            let mut vec = Vec::with_capacity(strings_len as _);

            for _ in 0..strings_len {
                vec.push(read_string(&mut self.reader)?);
            }

            vec
        };

        Ok(BookSheetRow {
            index: row_index,
            strings,
            empty: self.reader.read_u32::<LittleEndian>()? == 1,
            comment_out: self.reader.read_u32::<LittleEndian>()? == 1,
        })
    }

    pub fn read_next(&mut self) -> io::Result<BookSheet> {
        let rows_len = self.reader.read_u32::<LittleEndian>()?;
        let mut rows = Vec::with_capacity(rows_len as _);

        for _ in 0..rows_len {
            rows.push(self.read_next_row()?);
        }

        let sheet = BookSheet {
            rows,
            name: read_string(&mut self.reader)?,
            sheet_type: self.reader.read_u32::<LittleEndian>()?,
            text_length: self.reader.read_u32::<LittleEndian>()?,
            header_row: self.reader.read_u32::<LittleEndian>()?,
        };

        // Padding
        io::copy(&mut self.reader.by_ref().take(8), &mut io::sink())?;

        Ok(sheet)
    }
}

fn read_string(reader: &mut impl Read) -> io::Result<String> {
    let len = reader.read_u32::<LittleEndian>()?;
    let mut vec = vec![0_u8; len as usize];
    reader.read_exact(&mut vec)?;

    let padding = if len % 4 != 0 { 4 - len % 4 } else { 0 };
    io::copy(&mut reader.by_ref().take(padding as _), &mut io::sink())?;

    Ok(String::from_utf8_lossy(&vec).into())
}
