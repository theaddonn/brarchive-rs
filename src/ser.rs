use byteorder::WriteBytesExt;
use crate::{EntryDescriptor, Header, MAGIC, VERSIONS};
use crate::error::BrArchiveError;

pub fn write_header(buf: &mut Vec<u8>, header: &Header) -> Result<(), BrArchiveError>  {
    buf.write_u64(MAGIC)?;
    buf.write_u32(header.entries)?;

    if !VERSIONS.contains(&header.version) {
        return Err(BrArchiveError::UnsupportedVersion(header.version));
    }

    buf.write_u32(header.version)?;

    Ok(())
}

pub fn write_entry_descriptor(buf: &mut Vec<u8>, entry: &EntryDescriptor) -> Result<(), BrArchiveError>  {

}
