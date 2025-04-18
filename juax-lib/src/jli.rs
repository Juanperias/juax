use std::{fs::File, io::Write};
use thiserror::Error;

use crate::opcode::Opcode;

pub const JLI_MAGIC: u32 = 0x4A4C4900;

#[derive(Error, Debug)]
pub enum JliError {
    #[error("Io Error {0}")]
    IoError(#[from] std::io::Error),
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
        todo!();
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
