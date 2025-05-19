use std::{fs::Metadata, os::windows::fs::MetadataExt, time::SystemTime};

use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Eq, Serialize)]
pub struct FsEntry {
    name: Box<str>,
    last_modified: NaiveDate,
    ty: EntryType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
enum EntryType {
    File { size: usize },
    Dir {},
}

impl FsEntry {
    pub fn from_metadata(name: String, value: Metadata) -> Self {
        let last_write = value.modified().unwrap_or(SystemTime::UNIX_EPOCH);

        Self {
            name: name.into_boxed_str(),
            last_modified: DateTime::<Utc>::from(last_write).date_naive(),
            ty: if value.is_dir() {
                EntryType::Dir {}
            } else {
                EntryType::File {
                    size: value.file_size() as _,
                }
            },
        }
    }
}

impl PartialEq for FsEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.last_modified == other.last_modified && self.ty == other.ty {
            if self.name.as_ptr() == other.name.as_ptr() {
                return true;
            }
            return self.name == other.name;
        }
        false
    }
}
