use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, BufReader, Seek, SeekFrom, Write};
use crate::structs::{Block, Record, RecordStatus};
use crate::serial_file::SerialFile;

mod structs;
mod serial_file;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.bin")?;

    let block = Block {
        records: [
            (Record::new(1, "John Doe")),
            (Record::new(2, "Dohanes Joehan")),
            Record::new(-1, "")
        ],
    };

    block.write_to_file(&mut file);

    let mut serial_file = SerialFile::new(file);

    serial_file.write_record(Record::new(3, "Test Testing"))?;
    serial_file.write_record(Record::new(4, "Test Testingwasd"))?;
    serial_file.write_record(Record::new(5, "Test Testingawdawq"))?;


    serial_file.print_records()?;


    Ok(())
}