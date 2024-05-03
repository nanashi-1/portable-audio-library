use lz4::{Decoder, EncoderBuilder};

use super::Compression;
use crate::error::PortableAudioLibraryResult;
use std::io::{copy, Read, Write};

pub struct Lz4(pub u32);

impl<R, W> Compression<R, W> for Lz4
where
    R: Read,
    W: Write,
{
    fn compress(&self, reader: &mut R, writer: &mut W) -> PortableAudioLibraryResult<()> {
        let mut encoder = EncoderBuilder::new().level(self.0).build(writer)?;
        copy(reader, &mut encoder)?;

        Ok(())
    }

    fn decompress(&self, reader: &mut R, writer: &mut W) -> PortableAudioLibraryResult<()> {
        let mut decoder = Decoder::new(reader)?;
        copy(&mut decoder, writer)?;

        Ok(())
    }
}
