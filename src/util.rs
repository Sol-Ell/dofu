pub static WEBPAGE_FOLDER_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/pages/");
pub static UPLOADS_FOLDER_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/uploads/");

pub const FS_ENTRY_LIMIT: Option<usize> = Some(1000);
