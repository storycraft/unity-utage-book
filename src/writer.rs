use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::book::{
    header::BookHeader,
    sheet::{BookSheet, BookSheetRow},
};

#[derive(Debug)]
pub struct BookWriter<W: Write> {
    writer: W,
    sheet_count: u32,
    written_count: u32,
}

impl<W: Write> BookWriter<W> {
    pub fn create(mut writer: W, header: &BookHeader, sheet_count: u32) -> io::Result<Self> {
        writer.write_u32::<LittleEndian>(header.object_flag)?;
        writer.write_u32::<LittleEndian>(header.parent_object)?;
        writer.write_u32::<LittleEndian>(header.internal)?;
        writer.write_u32::<LittleEndian>(header.game_object)?;
        writer.write_u32::<LittleEndian>(header.enabled as u32)?;
        writer.write_u32::<LittleEndian>(header.editor_hide_flag)?;
        writer.write_u32::<LittleEndian>(header.script)?;
        write_string(&mut writer, &header.name)?;
        writer.write_u32::<LittleEndian>(header.import_version)?;
        writer.write_u32::<LittleEndian>(sheet_count)?;

        Ok(Self {
            writer,
            sheet_count,
            written_count: 0,
        })
    }

    pub fn sheet_count(&self) -> u32 {
        self.sheet_count
    }

    pub fn written_count(&self) -> u32 {
        self.written_count
    }

    fn write_next_row(&mut self, row: &BookSheetRow) -> io::Result<()> {
        self.writer.write_u32::<LittleEndian>(row.index)?;

        self.writer
            .write_u32::<LittleEndian>(row.strings.len() as _)?;
        for string in &row.strings {
            write_string(&mut self.writer, string)?;
        }

        self.writer.write_u32::<LittleEndian>(row.empty as _)?;
        self.writer
            .write_u32::<LittleEndian>(row.comment_out as _)?;

        Ok(())
    }

    pub fn write_next(&mut self, sheet: &BookSheet) -> io::Result<()> {
        self.writer
            .write_u32::<LittleEndian>(sheet.rows.len() as _)?;
        for row in &sheet.rows {
            self.write_next_row(row)?;
        }

        write_string(&mut self.writer, &sheet.name)?;
        self.writer.write_u32::<LittleEndian>(sheet.sheet_type)?;
        self.writer.write_u32::<LittleEndian>(sheet.text_length)?;
        self.writer.write_u32::<LittleEndian>(sheet.header_row)?;

        // Padding
        self.writer.write_all(&[0_u8; 8])?;

        self.written_count += 1;

        Ok(())
    }
}

fn write_string(writer: &mut impl Write, string: &str) -> io::Result<usize> {
    let len = string.len() as u32;

    writer.write_u32::<LittleEndian>(len)?;
    writer.write_all(string.as_bytes())?;

    if len % 4 != 0 {
        for _ in 0..(4 - len % 4) {
            writer.write(&[0])?;
        }
    }

    Ok(4 + string.len())
}
