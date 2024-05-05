use crate::{
    compression::{get_compression, CompressionType},
    error::PortableAudioLibraryResult,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Cursor, Read, Seek, Write},
    path::{Path, PathBuf},
};
use tempfile::tempfile;

/// Playlist name.
pub type Playlist = String;

const U64_SIZE: usize = std::mem::size_of::<u64>();
const PROGRESS_BAR_TEMPLATE: &str = "{spinner:.green} {msg} [{wide_bar}] {pos}/{len} ({eta})";
const PROGRESS_CHARS: &str = "=> ";
const CHECK_GREEN: &str = "\x1b[32m✓\x1b[0m";

/// Metadata of audio library.
#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub compression_type: CompressionType,
    pub audios: Vec<AudioMetadata>,
}

/// Metadata of audio.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AudioMetadata {
    pub name: String,
    pub size: u64,
    pub playlists: Vec<Playlist>,

    #[serde(skip_serializing, skip_deserializing)]
    pub path: PathBuf,
}

impl Metadata {
    /// Writes the metadata to the file.
    pub fn write_to_file(&mut self, path: impl Into<PathBuf>) -> PortableAudioLibraryResult<()> {
        let path = path.into();

        let compression = get_compression(&self.compression_type);
        let mut compressed_audio_files = vec![];

        let progress_bar = indicatif::ProgressBar::new(self.audios.len() as u64);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(PROGRESS_BAR_TEMPLATE)
                .unwrap()
                .progress_chars(PROGRESS_CHARS),
        );
        progress_bar.set_message("Compressing audio files");

        for audio in &mut self.audios {
            let mut audio_file = std::fs::File::open(&audio.path)?;
            let mut compressed_audio_file = tempfile()?;

            compression.compress(&mut audio_file, &mut compressed_audio_file)?;

            audio.size = compressed_audio_file.metadata()?.len();
            compressed_audio_files.push(compressed_audio_file);

            progress_bar.inc(1);
        }

        progress_bar.finish_and_clear();
        println!("{} Compression done!", CHECK_GREEN);

        let progress_bar = indicatif::ProgressBar::new(compressed_audio_files.len() as u64 + 1);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(PROGRESS_BAR_TEMPLATE)
                .unwrap()
                .progress_chars(PROGRESS_CHARS),
        );
        progress_bar.set_message("Writing to file");

        let mut portable_audio_library_file = std::fs::File::create(path)?;
        let serialized_portable_audio_library = bincode::serialize(&self)?;
        let metadata_size = serialized_portable_audio_library.len() as u64;
        portable_audio_library_file.write_all(&metadata_size.to_be_bytes())?;
        portable_audio_library_file.write_all(&serialized_portable_audio_library)?;
        progress_bar.inc(1);

        for mut compressed_audio_file in &compressed_audio_files {
            compressed_audio_file.seek(std::io::SeekFrom::Start(0))?;
            loop {
                let mut chunk = vec![0; 16 * 1024 * 1024]; // 16 MB chunk
                let bytes_read = compressed_audio_file.read(&mut chunk)?;

                if bytes_read == 0 {
                    break;
                }

                portable_audio_library_file.write_all(&chunk[..bytes_read])?;
            }

            progress_bar.inc(1);
        }

        progress_bar.finish_and_clear();
        println!("{} Writing done!", CHECK_GREEN);

        Ok(())
    }

    /// Reads the metadata from the file.
    pub fn read_from_file(
        path: impl Into<PathBuf>,
        directory_store: impl Into<PathBuf>,
    ) -> PortableAudioLibraryResult<Metadata> {
        let path = path.into();
        let directory_store: PathBuf = directory_store.into();

        let mut portable_audio_library_file = std::fs::File::open(path)?;

        let mut metadata_size_buf = vec![0; U64_SIZE];
        portable_audio_library_file.read_exact(&mut metadata_size_buf)?;
        let metadata_size = u64::from_be_bytes(metadata_size_buf.try_into().unwrap());

        let mut metadata_buf = vec![0; metadata_size as usize];
        portable_audio_library_file.read_exact(&mut metadata_buf)?;
        let mut metadata: Metadata = bincode::deserialize(&metadata_buf)?;

        let compression_type = &metadata.compression_type;
        let compression = get_compression(compression_type);

        let progress_bar = indicatif::ProgressBar::new(metadata.audios.len() as u64);
        progress_bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(PROGRESS_BAR_TEMPLATE)
                .unwrap()
                .progress_chars(PROGRESS_CHARS),
        );
        progress_bar.set_message("Decompressing audio files");

        let mut offset = metadata_size + U64_SIZE as u64;
        for audio_metadata in &mut metadata.audios {
            let mut buf = vec![0; audio_metadata.size as usize];
            portable_audio_library_file.seek(std::io::SeekFrom::Start(offset))?;
            portable_audio_library_file.read_exact(&mut buf)?;

            let path = directory_store.join(&audio_metadata.name);

            let mut audio_file = File::create(&path)?;

            compression.decompress(&mut Cursor::new(buf), &mut audio_file)?;

            audio_metadata.path = path;

            offset += audio_metadata.size;

            progress_bar.inc(1);
        }

        progress_bar.finish_and_clear();
        println!("{} Decompression done!", CHECK_GREEN);

        Ok(metadata)
    }
}
