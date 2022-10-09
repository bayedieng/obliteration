use crate::fs::file::File;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{Read, Seek, SeekFrom};
use util::mem::{read_array, read_u16_le, read_u64_le, read_u8, uninit};

// https://www.psdevwiki.com/ps4/SELF_File_Format
pub enum Executable {
    Little64(Little64),
}

impl Executable {
    pub fn load(mut file: File) -> Result<Self, LoadError> {
        // Read SELF header.
        let mut hdr: [u8; 32] = uninit();

        if let Err(e) = file.read_exact(&mut hdr) {
            return Err(LoadError::ReadSelfHeaderFailed(e));
        }

        let hdr = hdr.as_ptr();

        // Check magic.
        // Kyty also checking if Category = 0x01 & Program Type = 0x01 & Padding = 0x00.
        // Let's check only magic for now until something is broken.
        let magic: [u8; 8] = read_array(hdr, 0x00);
        let unknown = read_u16_le(hdr, 0x1a);

        if magic != [0x4f, 0x15, 0x3d, 0x1d, 0x00, 0x01, 0x01, 0x12] || unknown != 0x22 {
            return Err(LoadError::InvalidSelfMagic);
        }

        // Load header fields.
        let segments = read_u16_le(hdr, 0x18);

        // Load segment headers.
        for i in 0..segments {
            let mut hdr: [u8; 32] = uninit();

            if let Err(e) = file.read_exact(&mut hdr) {
                return Err(LoadError::ReadSelfSegmentHeaderFailed(i as _, e));
            }
        }

        // Read ELF header.
        let hdr_offset = file.stream_position().unwrap();
        let mut hdr: [u8; 16] = uninit();

        if let Err(e) = file.read_exact(&mut hdr) {
            return Err(LoadError::ReadElfHeaderFailed(e));
        }

        // Check magic.
        let hdr = hdr.as_ptr();
        let magic: [u8; 4] = read_array(hdr, 0x00);

        if magic != [0x7f, 0x45, 0x4c, 0x46] {
            return Err(LoadError::InvalidElfMagic);
        }

        // Load ELF header.
        let variant = match (read_u8(hdr, 0x04), read_u8(hdr, 0x05)) {
            (2, 1) => Self::Little64(Little64::load(file, hdr_offset)?),
            _ => return Err(LoadError::UnsupportedArchitecture),
        };

        Ok(variant)
    }
}

pub struct Little64 {}

impl Little64 {
    fn load(mut file: File, hdr_offset: u64) -> Result<Self, LoadError> {
        // Read remaining ELF header.
        let mut hdr: [u8; 48] = uninit();

        if let Err(e) = file.read_exact(&mut hdr) {
            return Err(LoadError::ReadElfHeaderFailed(e));
        }

        // Load remaining ELF header fields.
        let hdr = hdr.as_ptr();
        let e_phoff = read_u64_le(hdr, 0x20 - 0x10);
        let e_shoff = read_u64_le(hdr, 0x28 - 0x10);
        let e_phnum = read_u16_le(hdr, 0x38 - 0x10);
        let e_shnum = read_u16_le(hdr, 0x3c - 0x10);

        // Load program headers.
        file.seek(SeekFrom::Start(hdr_offset + e_phoff)).unwrap();

        for i in 0..e_phnum {
            // Read header.
            let mut hdr: [u8; 0x38] = uninit();

            if let Err(e) = file.read_exact(&mut hdr) {
                return Err(LoadError::ReadProgramHeaderFailed(i as _, e));
            }
        }

        // Load section headers.
        file.seek(SeekFrom::Start(hdr_offset + e_shoff)).unwrap();

        for i in 0..e_shnum {
            // Read header.
            let mut hdr: [u8; 64] = uninit();

            if let Err(e) = file.read_exact(&mut hdr) {
                return Err(LoadError::ReadSectionHeaderFailed(i as _, e));
            }
        }

        Ok(Self {})
    }
}

#[derive(Debug)]
pub enum LoadError {
    ReadSelfHeaderFailed(std::io::Error),
    InvalidSelfMagic,
    ReadSelfSegmentHeaderFailed(usize, std::io::Error),
    ReadElfHeaderFailed(std::io::Error),
    InvalidElfMagic,
    UnsupportedArchitecture,
    ReadProgramHeaderFailed(usize, std::io::Error),
    ReadSectionHeaderFailed(usize, std::io::Error),
}

impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ReadSelfHeaderFailed(e)
            | Self::ReadSelfSegmentHeaderFailed(_, e)
            | Self::ReadElfHeaderFailed(e)
            | Self::ReadProgramHeaderFailed(_, e)
            | Self::ReadSectionHeaderFailed(_, e) => Some(e),
            _ => None,
        }
    }
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::ReadSelfHeaderFailed(_) => f.write_str("cannot read SELF header"),
            Self::InvalidSelfMagic => f.write_str("invalid SELF magic"),
            Self::ReadSelfSegmentHeaderFailed(i, _) => {
                write!(f, "cannot read header for SELF segment #{}", i)
            }
            Self::ReadElfHeaderFailed(_) => f.write_str("cannot read ELF header"),
            Self::InvalidElfMagic => f.write_str("invalid ELF magic"),
            Self::UnsupportedArchitecture => f.write_str("unsupported architecture"),
            Self::ReadProgramHeaderFailed(i, _) => write!(f, "cannot read program header #{}", i),
            Self::ReadSectionHeaderFailed(i, _) => write!(f, "cannot read section header #{}", i),
        }
    }
}