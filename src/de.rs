use crate::error::BrArchiveError;
use crate::{EntryDescriptor, Header, ENTRY_NAME_LEN_MAX, MAGIC, VERSIONS};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn read_header(buf: &mut Cursor<&[u8]>) -> Result<Header, BrArchiveError> {
    let magic = buf.read_u64::<LittleEndian>()?;

    if magic != MAGIC {
        return Err(BrArchiveError::MagicMismatch(magic));
    }

    let entries = buf.read_u32::<LittleEndian>()?;

    let version = buf.read_u32::<LittleEndian>()?;

    if !VERSIONS.contains(&version) {
        return Err(BrArchiveError::UnsupportedVersion(version));
    }

    Ok(Header { entries, version })
}

pub fn read_entry_descriptor(buf: &mut Cursor<&[u8]>) -> Result<EntryDescriptor, BrArchiveError> {
    let name_len = buf.read_u8()?;

    if name_len > ENTRY_NAME_LEN_MAX as u8 {
        return Err(BrArchiveError::EntryNameTooLong(name_len))
    }

    let mut name = [0; ENTRY_NAME_LEN_MAX];
    buf.read_exact(&mut name)?;

    let name = String::from_utf8(name.to_vec()[0..name_len as usize].to_vec())?;

    let contents_offset = buf.read_u32::<LittleEndian>()?;
    let contents_len = buf.read_u32::<LittleEndian>()?;

    Ok(EntryDescriptor {
        name,
        contents_offset,
        contents_len,
    })
}

pub fn read_entry_contents(
    buf: &mut Cursor<&[u8]>,
    entry: &EntryDescriptor,
) -> Result<String, BrArchiveError> {
    let start_offset = buf.stream_position()?;

    buf.set_position(start_offset + entry.contents_offset as u64);

    let mut contents = vec![0; entry.contents_len as usize];
    buf.read_exact(contents.as_mut_slice())?;

    buf.set_position(start_offset);

    Ok(String::from_utf8(contents)?)
}
