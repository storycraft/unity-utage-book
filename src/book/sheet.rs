#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BookSheet {
    pub rows: Vec<BookSheetRow>,

    pub name: String,
    pub sheet_type: u32,
    pub text_length: u32,
    pub header_row: u32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BookSheetRow {
    pub index: u32,
    pub strings: Vec<String>,

    pub empty: bool,
    pub comment_out: bool
}