use crate::error::PortableAudioLibraryResult;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub mod gz;
pub mod lz4;
pub mod none;
pub mod snap;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum CompressionType {
    #[default]
    None,
    Lz4(u32),
    Snap,
    Gz(u32),
}

pub trait Compression<R, W>
where
    R: Read,
    W: Write,
{
    fn compress(&self, reader: &mut R, writer: &mut W) -> PortableAudioLibraryResult<()>;
    fn decompress(&self, reader: &mut R, writer: &mut W) -> PortableAudioLibraryResult<()>;
}

pub fn get_compression<R, W>(compression_type: &CompressionType) -> Box<dyn Compression<R, W>>
where
    R: Read,
    W: Write,
{
    match compression_type {
        CompressionType::None => Box::new(none::None),
        CompressionType::Lz4(level) => Box::new(lz4::Lz4(*level)),
        CompressionType::Snap => Box::new(snap::Snap),
        CompressionType::Gz(level) => Box::new(gz::Gz(*level)),
    }
}
