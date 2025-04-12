use crate::error::PackError;
use crate::header::PackHeader;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{Read, Seek};
use zip::ZipArchive;

pub mod error;
pub mod header;

/// Represents a pack, with its header and its content.
#[derive(Debug)]
pub struct Pack {
    /// The header of the pack
    pub header: PackHeader,
}

lazy_static! {
    static ref MOD_ID_REGEX: Regex =
        Regex::new(r#"^[a-zA-Z0-9_-]{6,63}$"#).expect("Cannot parse Mod ID regex");
}

pub fn parse_pack<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<Pack, error::PackError> {
    if archive.is_empty() {
        return Err(PackError::EmptyArchive);
    }

    // let file_names = archive.file_names().collect::<Vec<&str>>();

    // Search header file
    let config_file_index = archive.index_for_name("config.mp");
    let header = match config_file_index {
        Some(index) => {
            let mut file = archive.by_index(index).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).map_err(PackError::from)?;

            PackHeader::parse_from_buf(buf.as_slice())?
        }
        None => return Err(PackError::HeaderIsMissing),
    };

    Ok(Pack { header })
}

#[cfg(test)]
mod tests {
    use crate::parse_pack;
    use std::path::PathBuf;
    use zip::ZipArchive;

    #[test]
    fn parse_archive() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_file_path = manifest_dir.join("test").join("config.zip");

        let file = std::fs::File::open(test_file_path).expect("Cannot open test archive");
        let archive = ZipArchive::new(file).expect("Cannot open test archive");

        let pack = parse_pack(archive);
        // dbg!(&pack);
        assert!(pack.is_ok());
    }
}
