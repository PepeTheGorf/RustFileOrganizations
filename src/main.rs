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
            Some(Record{id: 1, name: "John Doe".parse().unwrap(), status: RecordStatus::ACTIVE }),
            Some(Record{id: 2, name: "Dohn Joe".parse().unwrap(), status: RecordStatus::ACTIVE }),
            Some(Record{id: 3, name: "Mohn Joe".parse().unwrap(), status: RecordStatus::ACTIVE }),

        ]
    };

    let block2 = Block {
        records: [
            Some(Record{id: 4, name: "Lohn Smoe".parse().unwrap(), status: RecordStatus::ACTIVE }),
            Some(Record{id: 5, name: "La Vaca Saturno Saturnita".parse().unwrap(), status: RecordStatus::ACTIVE }),
            None,
        ]
    };

    block.write_to_file(&mut file);
    block2.write_to_file(&mut file);

    let mut serial_file = SerialFile::new(file);

    serial_file.write_record(Record {
        id: 6, name: "New Record1".parse().unwrap(),
        status: RecordStatus::ACTIVE,
    })?;

    serial_file.write_record(Record {
        id: 7, name: "New Record2".parse().unwrap(),
        status: RecordStatus::ACTIVE,
    })?;

    println!("Record: {:?}", serial_file.find_record(6));

    serial_file.delete_logically(6)?;

    serial_file.print_file()?;

    Ok(())
}