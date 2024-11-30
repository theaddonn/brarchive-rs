use crate::de::{read_entry_contents, read_entry_descriptor, read_header};
use crate::error::BrArchiveError;
use std::collections::BTreeMap;
use std::io::Cursor;

pub(crate) mod de;
pub mod error;
pub(crate) mod ser;

pub(crate) const MAGIC: u64 = 0x267052A0B125277D;
pub(crate) const VERSIONS: [u32; 1] = [1];
pub(crate) const ENTRY_NAME_LEN_MAX: usize = 247;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Header {
    pub entries: u32,
    pub version: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct EntryDescriptor {
    pub name: String,
    pub contents_offset: u32,
    pub contents_len: u32,
}

pub fn serialize(data: impl Into<BTreeMap<String, String>>) -> Vec<u8> {
    let data = data.into();

    let buf = Vec::new();


}

pub fn deserialize(data: &[u8]) -> Result<BTreeMap<String, String>, BrArchiveError> {
    let mut cursor = Cursor::new(data);

    let header = read_header(&mut cursor)?;

    let mut entry_descriptors = Vec::with_capacity(header.entries as usize);

    for _ in 0..header.entries {
        let entry = read_entry_descriptor(&mut cursor)?;
        entry_descriptors.push(entry);
    }

    let mut entry_map = BTreeMap::new();

    for entry in entry_descriptors {
        let contents = read_entry_contents(&mut cursor, &entry)?;
        entry_map.insert(entry.name, contents);
    }

    Ok(entry_map)
}
