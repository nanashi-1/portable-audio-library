use crate::{
    error::PortableAudioLibraryResult,
    serialization::{AudioMetadata, Metadata, Playlist},
};
use std::{
    collections::HashMap,
    fs::DirEntry,
    path::{Path, PathBuf},
};

const ROOT: &str = "root";

pub fn build_metadata_from_directory(
    path: impl Into<PathBuf>,
) -> PortableAudioLibraryResult<Metadata> {
    let path = path.into();

    let mut metadata = Metadata::default();
    let mut hashmap = HashMap::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        match entry.file_type()? {
            file_type if file_type.is_dir() => {
                let playlist_name = entry.file_name().to_str().unwrap().to_string();

                for entry in std::fs::read_dir(entry.path())? {
                    let entry = entry?;

                    insert_audio_metadata_to_hashmap(entry, &playlist_name, &mut hashmap)?;
                }
            }
            file_type if file_type.is_file() => {
                insert_audio_metadata_to_hashmap(entry, &ROOT.to_string(), &mut hashmap)?;
            }
            _ => continue,
        }
    }

    hashmap.into_iter().for_each(|(_, hashmap_metadata)| {
        metadata.audios.push(hashmap_metadata);
    });

    Ok(metadata)
}

pub fn build_directory_from_metadata(
    path: impl Into<PathBuf>,
    metadata: &Metadata,
) -> PortableAudioLibraryResult<()> {
    let path = path.into();
    std::fs::create_dir_all(&path)?;

    for audio_metadata in &metadata.audios {
        write_audio_to_path(&path, audio_metadata)?;
    }

    Ok(())
}

fn write_audio_to_path(
    path: &Path,
    audio_metadata: &AudioMetadata,
) -> PortableAudioLibraryResult<()> {
    for playlist in &audio_metadata.playlists {
        match playlist == ROOT {
            true => {
                std::fs::copy(&audio_metadata.path, path.join(&audio_metadata.name))?;
            }
            false => {
                std::fs::create_dir_all(path.join(playlist))?;
                std::fs::copy(
                    &audio_metadata.path,
                    path.join(playlist).join(&audio_metadata.name),
                )?;
            }
        }
    }

    Ok(())
}

fn create_audio_metadata_from_entry(
    entry: DirEntry,
    playlist: &Playlist,
) -> PortableAudioLibraryResult<AudioMetadata> {
    let path = entry.path();
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let size = path.metadata()?.len();

    Ok(AudioMetadata {
        name,
        size,
        playlists: vec![playlist.to_string()],
        path,
    })
}

fn insert_audio_metadata_to_hashmap(
    entry: DirEntry,
    playlist_name: &Playlist,
    hashmap: &mut HashMap<String, AudioMetadata>,
) -> PortableAudioLibraryResult<()> {
    if entry.file_type()?.is_file() {
        let audio_metadata = create_audio_metadata_from_entry(entry, playlist_name)?;

        match hashmap.contains_key(&audio_metadata.name) {
            true => {
                let audio_metadata: &mut AudioMetadata =
                    hashmap.get_mut(&audio_metadata.name).unwrap();
                audio_metadata.playlists.push(playlist_name.clone());
            }
            false => {
                hashmap
                    .entry(audio_metadata.name.clone())
                    .or_insert_with(|| audio_metadata);
            }
        }
    }

    Ok(())
}
