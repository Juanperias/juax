use std::{
    fs::File,
    io::{Read, Write},
};
use thiserror::Error;

use crate::opcode::Opcode;

pub const JLI_MAGIC: u32 = 0x4A4C4900;

#[derive(Error, Debug)]
pub enum JliError {
    #[error("Io Error {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid Jli magic number")]
    InvalidMagic,
}

pub struct JliFile {
    pub content: Vec<u32>,
}

impl JliFile {
    pub const fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }
    pub fn write(&mut self, opcode: Opcode) {
        self.content.extend(opcode.to_bytes());
    }
    pub fn read(input: String) -> Result<Self, JliError> {
        let mut file = File::open(input)?;
        let magic = &mut [0u8; 4];
        file.read_exact(magic)?;

        if *magic != JLI_MAGIC.to_le_bytes() {
            return Err(JliError::InvalidMagic);
        }

        let mut raw = Vec::new();
        file.read_to_end(&mut raw)?;

        let code: Vec<u32> = raw
            .chunks(4)
            .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
            .collect();

        Ok(JliFile { content: code })
    }
    pub fn save(&self, output: String) -> Result<(), JliError> {
        let mut file = File::create(output)?;

        file.write_all(&JLI_MAGIC.to_le_bytes())?;
        for number in &self.content {
            file.write_all(&number.to_le_bytes())?;
        }

        Ok(())
    }
}
