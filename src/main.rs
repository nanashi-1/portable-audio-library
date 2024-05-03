use clap::{Parser, Subcommand, ValueEnum};
use portable_audio_library::{
    builder::directory::{self},
    compression,
    error::PortableAudioLibraryResult,
    serialization::Metadata,
};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    #[clap(about = "Encode a .pal file.")]
    Encode {
        #[arg()]
        input: String,

        #[arg()]
        output: String,

        #[clap(
            short,
            long,
            default_value = "directory",
            help = "Refers to what kind of audio library that needs to be converted to a .pal file."
        )]
        builder: BuilderType,

        #[clap(
            short = 't',
            long,
            default_value = "none",
            help = "Type of compression to apply to the audio files."
        )]
        compression_type: CompressionType,

        #[clap(
            short = 'l',
            long,
            default_value = "0",
            help = "Level of compression. This only works in some compression types."
        )]
        compression_level: u32,
    },

    #[clap(about = "Decode a .pal file.")]
    Decode {
        #[arg()]
        input: String,

        #[arg()]
        output: String,

        #[clap(
            short,
            long,
            default_value = "directory",
            help = "Refers to what kind of audio library that needs to be converted from a .pal file."
        )]
        builder: BuilderType,
    },
}

#[derive(ValueEnum, Clone, Debug, Default)]
enum BuilderType {
    #[default]
    Directory,
}

#[derive(ValueEnum, Clone)]
enum CompressionType {
    None,
    Lz4,
    Snap,
    Gz,
}

impl CompressionType {
    fn into(&self, compression_level: u32) -> compression::CompressionType {
        match self {
            CompressionType::None => compression::CompressionType::None,
            CompressionType::Lz4 => compression::CompressionType::Lz4(compression_level),
            CompressionType::Snap => compression::CompressionType::Snap,
            CompressionType::Gz => compression::CompressionType::Gz(compression_level),
        }
    }
}

fn main() -> PortableAudioLibraryResult<()> {
    let cli = Cli::parse();

    match &cli.subcommand {
        Subcommands::Encode {
            input,
            output,
            builder,
            compression_type,
            compression_level,
        } => match builder {
            BuilderType::Directory => {
                let mut metadata = directory::build_metadata_from_directory(input)?;
                metadata.compression_type = compression_type.into(*compression_level);

                metadata.write_to_file(output)?;
            }
        },
        Subcommands::Decode {
            input,
            output,
            builder,
        } => {
            let directory_store = tempfile::tempdir()?;
            let metadata = Metadata::read_from_file(input, directory_store.path())?;

            match builder {
                BuilderType::Directory => {
                    directory::build_directory_from_metadata(output, &metadata)?;
                }
            }
        }
    }

    Ok(())
}
