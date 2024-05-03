use flate2::read::{GzDecoder, GzEncoder};

use super::Compression;
use std::io::{copy, Read, Write};

pub struct Gz(pub u32);

impl<R, W> Compression<R, W> for Gz
where
    R: Read,
    W: Write,
{
    fn compress(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> crate::error::PortableAudioLibraryResult<()> {
        let mut encoder = GzEncoder::new(reader, flate2::Compression::new(self.0));
        copy(&mut encoder, writer)?;

        Ok(())
    }

    fn decompress(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> crate::error::PortableAudioLibraryResult<()> {
        let mut decoder = GzDecoder::new(reader);
        copy(&mut decoder, writer)?;

        Ok(())
    }
}
