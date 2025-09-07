use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{Read, Write};
use std::ptr::write;
use serde::{Deserialize, Serialize};
use crate::structs::RecordStatus::ACTIVE;


pub const LAST_RECORD: i8 = -1;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RecordStatus {
    ACTIVE,
    DISABLED
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub name: [u8; 32],
    pub status: RecordStatus
}

impl Record {
    pub fn new(id: i32, name: &str) -> Self {
        let mut buf = [0u8; 32];
        let bytes = name.as_bytes();
        buf[..bytes.len()].copy_from_slice(&bytes[..bytes.len()]);

        Record {
            id,
            name: buf,
            status: ACTIVE
        }
    }

    pub fn name(&self) -> String {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        String::from_utf8_lossy(&self.name[..end]).to_string()
    }

    pub fn is_last(&self) -> bool {
        self.id == LAST_RECORD as i32
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub records: [Record; 3]
}

impl Block {

    pub fn new() -> Self {
        Block {
            records: [
                Record::new(-1, ""),
                Record::new(-1, ""),
                Record::new(-1, "")
            ]
        }
    }
    pub fn write_to_file(&self, file: &mut File) {
        let enc = bincode::serialize(&self).unwrap();

        file.write_all(&enc).unwrap();
    }

    pub fn read_from_file(file: &mut File) -> Self {
        let block_size = size_of::<Block>();
        let mut buf = vec![0u8; block_size];

        file.read_exact(&mut buf).unwrap();

        bincode::deserialize::<Block>(&buf).unwrap()
    }
}
