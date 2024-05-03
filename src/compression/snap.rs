use super::Compression;
use snap::read::{FrameDecoder, FrameEncoder};
use std::io::{copy, Read, Write};

pub struct Snap;

impl<R, W> Compression<R, W> for Snap
where
    R: Read,
    W: Write,
{
    fn compress(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> crate::error::PortableAudioLibraryResult<()> {
        let mut encoder = FrameEncoder::new(reader);
        copy(&mut encoder, writer)?;

        Ok(())
    }

    fn decompress(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> crate::error::PortableAudioLibraryResult<()> {
        let mut decoder = FrameDecoder::new(reader);
        copy(&mut decoder, writer)?;

        Ok(())
    }
}
