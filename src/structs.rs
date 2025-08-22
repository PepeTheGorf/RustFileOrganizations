use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{Read, Write};
use std::ptr::write;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RecordStatus {
    ACTIVE,
    DISABLED
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: i32,
    pub name: String,
    pub status: RecordStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub records: [Option<Record>; 3]
}

impl Block {
    pub fn write_to_file(&self, file: &mut File) {
        let enc = bincode::serialize(&self).unwrap();
        let len = enc.len() as i64;

        file.write_all(&len.to_le_bytes()).unwrap();
        file.write_all(&enc).unwrap();
    }

    pub fn read_from_file(file: &mut File) -> (u64, Self) {
        let mut len_buf = [0u8; 8];
        file.read_exact(&mut len_buf).expect("TODO: panic message");

        let block_len = u64::from_le_bytes(len_buf);
        let mut buf = vec![0u8; block_len as usize];
        file.read_exact(&mut buf).expect("TODO: panic message");

        (block_len, bincode::deserialize::<Block>(&buf).unwrap())
    }

    pub fn read_from_file_exact(file: &mut File, buf_len: [u8; 8]) -> Self {
        let block_len = u64::from_le_bytes(buf_len);
        let mut buf = vec![0u8; block_len as usize];
        file.read_exact(&mut buf).expect("TODO: panic message");

        bincode::deserialize::<Block>(&buf).unwrap()
    }
}
