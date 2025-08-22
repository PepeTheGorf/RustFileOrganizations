use crate::structs::{Block, Record, RecordStatus};
use std::fs::File;
use std::io;
use std::io::{Error, Read, Seek, SeekFrom};

pub struct SerialFile {
    pub file: File,
}

impl SerialFile {
    pub fn new(file: File) -> Self {
        Self {
            file
        }
    }

    pub fn find_record(&mut self, record_id: i32) -> Option<Record> {
        self.file.seek(SeekFrom::Start(0)).expect("Unable to seek...");

        loop {
            let mut len_buf = [0u8; 8];
            self.file.read_exact(&mut len_buf).unwrap();

            let block_len = u64::from_le_bytes(len_buf);
            let mut buf = vec![0u8; block_len as usize];
            self.file.read_exact(&mut buf).unwrap();

            match bincode::deserialize::<Block>(&buf) {
                Ok(block) => {
                    for rec in block.records {
                        if let Some(record) = rec {
                            if record.id == record_id {
                                if record.status == RecordStatus::DISABLED {
                                    println!("Record with ID: {} is disabled!\n", record_id);
                                    return None;
                                }
                                return Some(record);
                            }
                        } else {
                            //println!("Record with ID: {} does not exist!\n", record_id);
                            return None;
                        }
                    }
                }
                Err(_) => {
                    println!("Error...");
                    break;
                }
            }

        }
        None
    }

    pub fn write_record(&mut self, record: Record) -> io::Result<()> {

        let rec = self.find_record(record.id);

        if rec.is_some() {
            return Err(Error::from(io::ErrorKind::AlreadyExists))
        }

        self.file.seek(SeekFrom::Start(0))?;

        let mut last_block: Option<Block> = None;
        let mut last_block_len = -1;
        loop {
            let mut len_buf = [0u8; 8];
            if let Err(e) = self.file.read_exact(&mut len_buf) {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(e);
                }
            }
            last_block = Some(Block::read_from_file_exact(&mut self.file, len_buf));
            last_block_len = u64::from_le_bytes(len_buf) as i64;
        }
        last_block_len += 8;
        if let Some(mut block) = last_block {
            for position in 0..block.records.len() {
                if block.records[position].is_none() {
                    self.file.seek(SeekFrom::Current(-last_block_len))?;

                    block.records[position] = Some(record);

                    if position == block.records.len() - 1 {
                        let new_block = Block {
                            records: [
                                None,
                                None,
                                None,
                            ]
                        };

                        block.write_to_file(&mut self.file);
                        new_block.write_to_file(&mut self.file);
                    } else {
                        block.records[position + 1] = None;

                        block.write_to_file(&mut self.file);
                    }
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn print_file(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;

        let mut block_index = 0;
        loop {
            println!("Block index: {}", block_index);

            let read = Block::read_from_file(&mut self.file);
            let block = read.1;
            for record in block.records {
                match record {
                    None => {
                        println!("\t *****THE END*****");
                        return Ok(())
                    }
                    Some(rec) => {
                        if rec.status == RecordStatus::ACTIVE {
                            println!("\t ID: {}, Name: {}", rec.id, rec.name);
                        }
                    }
                }
            }
            block_index+=1;
        }
    }

    pub fn delete_logically(&mut self, record_id: i32) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;

        let mut last_block_len = 0;
        loop {
            let mut read = Block::read_from_file(&mut self.file);
            let mut block = read.1;

            last_block_len = read.0;
            for position in 0..block.records.len() {
                if let Some(record) = block.records[position].as_mut() {
                    if record.id == record_id {
                        record.status = RecordStatus::DISABLED;

                        self.file.seek(SeekFrom::Current(-((last_block_len + 8) as i64)))?;
                        block.write_to_file(&mut self.file);
                        return Ok(())
                    }
                } else {
                    return Err(Error::from(io::ErrorKind::UnexpectedEof))
                }
            }
        }
    }
}
