use crate::de::{read_entry_contents, read_entry_descriptor, read_header};
use crate::error::BrArchiveError;
use std::collections::BTreeMap;
use std::io::Cursor;
use crate::ser::{write_entry_contents, write_entry_descriptor, write_header};

pub(crate) mod de;
pub mod error;
pub(crate) mod ser;

pub(crate) const MAGIC: u64 = 0x267052A0B125277D;
pub(crate) const VERSIONS: [u32; 1] = [1];
pub(crate) const ENTRY_NAME_LEN_MAX: usize = 247;
pub(crate) const HEADER_SIZE: usize = 16;
pub(crate) const ENTRY_SIZE: usize = 256;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Header {
    pub entries: u32,
    pub version: u32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct EntryDescriptor<'a> {
    pub name: &'a str,
    pub contents_offset: u32,
    pub contents_len: u32,
}

pub fn serialize(data: impl IntoIterator<Item = (String, String)>) -> Result<Vec<u8>, BrArchiveError> {
    let data = data.into_iter().collect::<Vec<_>>();

    let mut buf = Vec::new();
    let mut buf_pos = 0;

    let header = Header {
        entries: data.len() as u32,
        version: 1,
    };

    write_header(&mut buf, &header)?;
    buf_pos += HEADER_SIZE;
    buf_pos += ENTRY_SIZE * data.len();

    for (name, content) in &data {
        let entry = EntryDescriptor {
            name,
            contents_offset: buf_pos as u32,
            contents_len: content.len() as u32,
        };
        
        write_entry_descriptor(&mut buf, &entry)?;
    }

    for (_, content) in &data {
        write_entry_contents(&mut buf, content)?;
    }

    Ok(buf)
}

pub fn deserialize(data: &[u8]) -> Result<BTreeMap<String, String>, BrArchiveError> {
    let mut buf = Cursor::new(data);

    let header = read_header(&mut buf)?;

    let mut entry_descriptors = Vec::with_capacity(header.entries as usize);

    for _ in 0..header.entries {
        let entry = read_entry_descriptor(&mut buf)?;
        entry_descriptors.push(entry);
    }

    let mut entry_map = BTreeMap::new();

    for entry in entry_descriptors {
        let contents = read_entry_contents(&mut buf, &entry)?;
        entry_map.insert(entry.name.to_string(), contents);
    }

    Ok(entry_map)
}
