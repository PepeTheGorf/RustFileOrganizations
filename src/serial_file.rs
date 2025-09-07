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

    pub fn write_record(&mut self, record: Record) -> io::Result<()> {
        self.file.seek(SeekFrom::End(-(size_of::<Block>() as i64)))?;

        let mut last_block: Block = Block::read_from_file(&mut self.file);
        for position in 0..last_block.records.len() {
            if last_block.records[position].is_last() {
                last_block.records[position] = record;

                if position == last_block.records.len() - 1 {
                    let mut new_last_block = Block::new();

                    self.file.seek(SeekFrom::End(-(size_of::<Block>() as i64)))?;
                    last_block.write_to_file(&mut self.file);
                    new_last_block.write_to_file(&mut self.file);
                } else {
                    last_block.records[position + 1] = Record::new(-1, "");

                    self.file.seek(SeekFrom::End(-(size_of::<Block>() as i64)))?;
                    last_block.write_to_file(&mut self.file);
                }
                break;
            }
        }
        Ok(())
    }

    pub fn print_records(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut block_index = 0;

        loop {
            let block = Block::read_from_file(&mut self.file);
            let mut found_last = false;

            println!("Block with index: {}", block_index);
            for record in block.records {
                if record.is_last() {
                    found_last = true;
                    break;
                }

                println!("\tRecord with ID: {} is: {}", record.id, record.name());
            }

            block_index += 1;

            if found_last {
                break;
            }
        }
        Ok(())
    }
}
