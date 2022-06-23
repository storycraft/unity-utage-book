#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BookHeader {
    pub object_flag: u32,
    pub parent_object: u32,
    pub internal: u32,

    pub game_object: u32,
    pub enabled: bool,
    pub editor_hide_flag: u32,
    pub script: u32,

    pub name: String,
    
    pub import_version: u32
}