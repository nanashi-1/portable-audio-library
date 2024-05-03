use crate::{
    error::PortableAudioLibraryResult,
    serialization::{AudioMetadata, Metadata},
};
use std::{collections::HashMap, path::PathBuf};

/// Builds a metadata object from a .m3u file.
pub fn build_metadata_from_m3u(path: impl Into<PathBuf>) -> PortableAudioLibraryResult<Metadata> {
    let path = path.into();

    let mut metadata = Metadata::default();
    let mut hashmap = HashMap::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let playlist_name = entry.file_name().to_string_lossy().to_string();

        if entry
            .path()
            .extension()
            .map(|ext| ext == "m3u")
            .unwrap_or(false)
        {
            let content = std::fs::read_to_string(entry.path())?;
            let audios: Vec<&str> = content.split('\n').collect();

            for audio in audios {
                let path = PathBuf::from(audio);
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                let size = path.metadata()?.len();

                match hashmap.contains_key(&name) {
                    true => {
                        let audio: &mut AudioMetadata = hashmap.get_mut(&name).unwrap();
                        audio.playlists.push(playlist_name.clone());
                    }
                    false => {
                        hashmap.insert(
                            name.clone(),
                            AudioMetadata {
                                path,
                                name,
                                size,
                                playlists: vec![playlist_name.clone()],
                            },
                        );
                    }
                }
            }
        }
    }

    for (_, audio) in hashmap {
        metadata.audios.push(audio);
    }

    Ok(metadata)
}

/// Builds a .m3u file from a metadata object.
pub fn build_m3u_from_metadata(
    path: impl Into<PathBuf>,
    metadata: &Metadata,
) -> PortableAudioLibraryResult<()> {
    let path: PathBuf = path.into();
    std::fs::create_dir_all(&path)?;

    let mut hashmap = HashMap::new();

    for audio in &metadata.audios {
        let audio_path = path.join(&audio.name);
        std::fs::copy(&audio.path, audio_path.clone())?;

        for playlist in &audio.playlists {
            let playlist_path = path.join(playlist).with_extension("m3u");

            match hashmap.contains_key(&playlist_path) {
                true => {
                    let content: &mut String = hashmap.get_mut(&playlist_path).unwrap();
                    content.push('\n');
                    content.push_str(audio_path.to_string_lossy().as_ref());
                }
                false => {
                    hashmap.insert(playlist_path, audio_path.to_string_lossy().to_string());
                }
            }
        }
    }

    for (playlist_path, content) in hashmap {
        std::fs::write(playlist_path, content)?;
    }

    Ok(())
}
