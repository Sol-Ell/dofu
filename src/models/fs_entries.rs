use std::sync::Arc;

use serde::{Serialize, ser::SerializeSeq};

use super::FsEntry;

#[derive(Debug, Clone)]
pub struct FsEntries(Arc<[FsEntry]>);

impl FsEntries {}

impl From<Vec<FsEntry>> for FsEntries {
    fn from(value: Vec<FsEntry>) -> Self {
        FsEntries(Arc::from(value.into_boxed_slice()))
    }
}

impl Eq for FsEntries {}

impl PartialEq for FsEntries {
    fn eq(&self, other: &Self) -> bool {
        if self.0.as_ptr() == other.0.as_ptr() {
            return true;
        }

        let slice_1: &[_] = &self.0;
        let slice_2: &[_] = &other.0;

        let len_1 = slice_1.len();
        let len_2 = slice_2.len();

        if len_1 == len_2 {
            let mut i = 0;
            while i < len_2 {
                if slice_1[i] != slice_2[i] {
                    return false;
                }
                i += 1;
            }
            return true;
        }

        false
    }
}

impl Serialize for FsEntries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for entry in self.0.iter() {
            seq.serialize_element(entry)?;
        }

        seq.end()
    }
}
