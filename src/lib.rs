//! This is an API for creation and manipulation of Portable Audio Library (PAL) files.
//!
//! # Usage
//!
//! ## Create a new PAL file from an audio library directory
//!
//! ```
//! use portable_audio_library::builder::directory::build_metadata_from_directory;
//!
//! let metadata = build_metadata_from_directory("doc-tests/example-library").unwrap();
//! ```
//!
//! ## Create a new PAL file from a metadata
//!
//! ```
//! use portable_audio_library::builder::directory::build_metadata_from_directory;
//!
//! let mut metadata = build_metadata_from_directory("doc-tests/example-library").unwrap();
//! metadata.write_to_file("doc-tests/example-library.pal").unwrap();
//! ```
//!
//! ## Create an audio library directory from a PAL file
//!
//! ```
//! use portable_audio_library::{tempfile, serialization::Metadata};
//! use portable_audio_library::builder::directory::build_directory_from_metadata;
//!
//! let temporary_directory = tempfile::tempdir().unwrap();
//! let metadata = Metadata::read_from_file("doc-tests/example-library.pal", temporary_directory.path()).unwrap();
//!
//! build_directory_from_metadata("doc-tests/out-library", &metadata).unwrap();
//! ```
//

pub use tempfile;

pub mod builder;
pub mod compression;
pub mod error;
pub mod serialization;
