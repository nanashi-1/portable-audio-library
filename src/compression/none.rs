use super::Compression;
use crate::error::PortableAudioLibraryResult;
use std::io::{copy, Read, Write};

pub struct None;

impl<R, W> Compression<R, W> for None
where
    R: Read,
    W: Write,
{
    fn compress(&self, reader: &mut R, writer: &mut W) -> PortableAudioLibraryResult<()> {
        copy(reader, writer)?;

        Ok(())
    }

    fn decompress(&self, reader: &mut R, writer: &mut W) -> PortableAudioLibraryResult<()> {
        copy(reader, writer)?;

        Ok(())
    }
}
