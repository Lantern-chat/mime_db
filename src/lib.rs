#![warn(clippy::perf)]

use unicase::UniCase;

#[derive(Debug, Clone, Copy)]
pub struct MimeEntry {
    pub compressible: bool,
    pub extensions: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
pub struct ExtEntry {
    pub types: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/mime_db.rs"));

pub fn lookup_ext(ext: &str) -> Option<&ExtEntry> {
    EXT_TO_MIME.get(&UniCase::new(ext))
}

pub fn lookup_mime(mime: &str) -> Option<&MimeEntry> {
    MIME_TO_EXT.get(&UniCase::new(mime))
}

pub fn list_mimes() -> impl Iterator<Item = (&'static str, &'static MimeEntry)> {
    MIME_TO_EXT.into_iter().map(|(k, v)| (k as &str, v))
}

#[inline]
pub fn lookup_mime_from_ext(ext: &str) -> Option<&MimeEntry> {
    let entry = lookup_ext(ext)?;

    if entry.types.is_empty() {
        return None;
    }

    // Lookup IANA entry
    lookup_mime(entry.types[0])
}

/// https://en.wikipedia.org/wiki/List_of_file_signatures
pub fn from_prefix(bytes: &[u8]) -> Option<(&str, Option<&MimeEntry>)> {
    // TODO: Convert this into a trie
    #[rustfmt::skip]
    static MAGIC_BYTES: &[(&[u8], &str)] = &[
        (b"\x89PNG\r\n\x1a\n", "image/png"),
        (&[0xff, 0xd8, 0xff], "image/jpeg"),
        (&[0xCF, 0x84, 0x01], "image/jpeg"),
        (b"GIF89a", "image/gif"),
        (b"GIF87a", "image/gif"),
        (b"MM\x00*", "image/tiff"),
        (b"II*\x00", "image/tiff"),
        (b"DDS ", "image/vnd.ms-dds"),
        (b"BM", "image/bmp"),
        (&[0, 0, 1, 0], "image/x-icon"),
        (&[0x69, 0x63, 0x6e, 0x73], "image/x-icns"),
        (b"#?RADIANCE", "image/vnd.radiance"),
        (b"P1", "image/x-portable-anymap"),
        (b"P2", "image/x-portable-anymap"),
        (b"P3", "image/x-portable-anymap"),
        (b"P4", "image/x-portable-anymap"),
        (b"P5", "image/x-portable-anymap"),
        (b"P6", "image/x-portable-anymap"),
        (b"P7", "image/x-portable-anymap"),
        (b"farbfeld", "image/x-farbfeld"),
        (b"\0\0\0 ftypavif", "image/avif"),
        (b"\0\0\0\x1cftypisom", "video/mp4"),
        (b"\0\0\0\x1cftypMSNV", "video/mp4"),
        (b"\0\0\0\x1cftypmmp4", "video/mp4"),
        (&[0x66, 0x74, 0x79, 0x70, 0x68, 0x65, 0x69, 0x63, 0x66, 0x74, 0x79, 0x70, 0x6d], "image/heic"),
        (&[0x76, 0x2f, 0x31, 0x01], "image/x-exr"), // = &exr::meta::magic_number::BYTES
        (&[0x38, 0x42, 0x50, 0x53], "image/vnd.adobe.photoshop"),
        (&[0x25, 0x50, 0x44, 0x46, 0x2D], "application/pdf"),
        (&[0x4F, 0x67, 0x67, 0x53], "audio/ogg"),
        (&[0xFF, 0xFB], "audio/mp3"),
        (&[0xFF, 0xF3], "audio/mp3"),
        (&[0xFF, 0xF2], "audio/mp3"),
        (&[0xFF, 0x0A], "image/jxl"),
        (&[0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x4C, 0x20, 0x0D, 0x0A, 0x87, 0x0A], "image/jxl"),
        (&[0x49, 0x44, 0x33], "audio/mp3"),
        (&[0x4F, 0x54, 0x54, 0x4F], "font/otf"),
        (&[0x00, 0x01, 0x00, 0x00, 0x00], "font/ttf"),
        (&[0x66, 0x4C, 0x61, 0x43], "audio/x-flac"),
        (
            &[0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x4C, 0x20, 0x0D, 0x0A, 0x87, 0x0A],
            "image/jxl",
        ),
        (&[0x4D, 0x54, 0x68, 0x64], "audio/midi"),
        (
            &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1],
            "application/msword",
        ),
        (&[0x1F, 0x8B], "application/gzip"),

        (
            &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C],
            "application/x-7z-compressed",
        ),
        (&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00], "application/x-xz"),
        (&[0x46, 0x4C, 0x49, 0x46], "image/flif"),
        (&[0x1A, 0x45, 0xDF, 0xA3], "video/x-matroska"),
        (&[0x47], "video/mpeg"),

        (&[0x78, 0x01], "application/z-lib"),
        (&[0x78, 0x5E], "application/z-lib"),
        (&[0x78, 0x9C], "application/z-lib"),
        (&[0x78, 0xDA], "application/z-lib"),
        (&[0x78, 0x20], "application/z-lib"),
        (&[0x78, 0x7D], "application/z-lib"),
        (&[0x78, 0xBB], "application/z-lib"),
        (&[0x78, 0xF9], "application/z-lib"),
        (b"FLhd", "application/vnd.fl-studio"),
        (b"#EXTM3U", "audio/mpegurl"),
        (b"BZh", "application/x-bzip2"),
        (
            &[0x42, 0x4C, 0x45, 0x4E, 0x44, 0x45, 0x52],
            "application/x-blend",
        ),
        (&[0x46, 0x4C, 0x56], "video/x-flv"),
        (&[0x4D, 0x53, 0x43, 0x46], "application/vnd.ms-cab-compressed"),
        (
            &[0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C],
            "video/x-ms-wmv",
        ),
        (
            &[
                0x53, 0x49, 0x4D, 0x50, 0x4C, 0x45, 0x20, 0x20, 0x3D, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x54,
            ],
            "image/fits",
        ),
        (
            &[0x06, 0x06, 0xED, 0xF5, 0xD8, 0x1D, 0x46, 0xE5, 0xBD, 0x31, 0xEF, 0xE7, 0xFE, 0x74, 0xB7, 0x1D],
            "application/x-indesign",
        ),
    ];

    static MAGIC_BYTES_WITH_OFFSETS: &[(usize, &[u8], &str)] = &[
        (4, b"ftyp3g", "video/3gpp"),
        (
            257,
            &[0x75, 0x73, 0x74, 0x61, 0x72, 0x00, 0x30, 0x30],
            "application/tar",
        ),
        (
            257,
            &[0x75, 0x73, 0x74, 0x61, 0x72, 0x20, 0x20, 0x00],
            "application/tar",
        ),
    ];

    const RIFFS: &[(&[u8], &str)] = &[
        (&[0x57, 0x45, 0x42, 0x50], "image/webp"),
        (&[0x57, 0x41, 0x56, 0x45], "audio/wav"),
        (&[0x41, 0x56, 0x49, 0x20], "video/x-msvideo"),
        (&[0x43, 0x44, 0x44, 0x41], "audio/cda"),
        (&[0x41, 0x56, 0x49, 0x20], "video/avi"),
    ];

    for (prefix, mime) in MAGIC_BYTES {
        if bytes.starts_with(prefix) {
            return Some((mime, lookup_mime(mime)));
        }
    }

    if bytes.starts_with(b"RIFF") && bytes.len() >= 12 {
        let bytes = &bytes[8..];
        for (prefix, mime) in RIFFS {
            if bytes.starts_with(prefix) {
                return Some((*mime, lookup_mime(mime)));
            }
        }
    }

    for (offset, prefix, mime) in MAGIC_BYTES_WITH_OFFSETS {
        if bytes.len() > *offset && bytes[*offset..].starts_with(prefix) {
            return Some((mime, lookup_mime(mime)));
        }
    }

    None
}
