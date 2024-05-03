# Portable Audio Library

[![Clippy](https://github.com/nanashi-1/portable-audio-library/actions/workflows/check.yml/badge.svg)](https://github.com/nanashi-1/portable-audio-library/actions/workflows/check.yml)
[![Crates.io](https://img.shields.io/crates/v/portable-audio-library.svg)](https://crates.io/crates/portable-audio-library)
[![Rustdoc](https://img.shields.io/badge/doc-rustdoc-green.svg)](https://docs.rs/portable-audio-library/0.1.0/portable_audio_library)

> [!NOTE]
> This project is still in alpha.

A portable audio library file format.

## Planned Builders

Builders are a group functions that converts a certain audio library format into Portable Audio Library and vice versa.

- [x] Directory Audio Library
- [ ] M3U
- [ ] PLS
- [ ] XSPF

## Usage

Get basic usage using,

    portable-audio-library help

## Examples

Convert a audio library directory into a `.pal` file,

    portable-audio-library encode /path/to/audio-library audio-library.pal

Apply compression([`snap`](https://github.com/BurntSushi/rust-snappy)),

    portable-audio-library encode /path/to/audio-library audio-library-compressed.pal -t snap

Convert a `.pal` file into a audio library directory,

    portable-audio-library decode audio-library.pal /path/to/audio-library

## License

This project is licensed under the MIT license.
