/*
* TODO:
*   Issues:
*      possible issue if the data file is bigger then 64bit length (very unlikely)
*   Improvements:
*       Split up the dat files to reduce single file size
*       Add compression
*       Cache the idx file so it only has to be read once
*/

use binary_rw::{BinaryReader, BinaryWriter, OpenType};
use std::fs;
use std::fs::Metadata;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

struct MetaFile {
    pub metadata: Metadata,
    pub file: String,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn recursive_search(path: &str, data: &mut Vec<MetaFile>) {
    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();

        if dir.metadata().unwrap().is_dir() {
            recursive_search(&dir.path().into_os_string().into_string().unwrap(), data);
        } else {
            data.push(MetaFile {
                metadata: dir.metadata().unwrap(),
                file: dir.path().into_os_string().into_string().unwrap(),
            });
        }
    }
}

pub fn compile_assets() {
    println!("building assets");

    if path_exists("./data.idx") {
        println!("deleting data.idx");
        fs::remove_file("./data.idx").unwrap();
    }

    if path_exists("./data.dat") {
        println!("deleting data.dat");
        fs::remove_file("./data.dat").unwrap();
    }

    println!("Setting up writers");

    let mut curr_data_loc: u64 = 0;
    let mut idx_writer = BinaryWriter::new("data.idx", OpenType::OpenAndCreate);
    let mut dat_writer = BinaryWriter::new("data.dat", OpenType::OpenAndCreate);

    let mut files: Vec<MetaFile> = Vec::new();

    recursive_search("./res", &mut files);

    idx_writer.write_u64(files.len() as u64);

    for file in files {
        println!(
            "Building file, Name: {}, Size: {}",
            file.file,
            file.metadata.len()
        );

        let mut file_reader = BinaryReader::new(&file.file, OpenType::Open);

        let mut buffer = file_reader.read_bytes(file.metadata.len());

        dat_writer.write_bytes(buffer);

        // Write idx entry
        {
            // write file size
            idx_writer.write_u64(file.metadata.len());
            // write file offset
            idx_writer.write_u64(curr_data_loc);
            // write file name
            idx_writer.write_string(file.file);
        }

        curr_data_loc = curr_data_loc + file.metadata.len() as u64;
    }
}

pub fn get_asset(asset_path: &str) -> Vec<u8> {
    // open idx and find file
    let mut reader = BinaryReader::new("data.idx", OpenType::Open);

    let files = reader.read_u64();

    let mut file_size: u64 = 0;
    let mut file_offset: u64 = 0;
    let mut file_name: String = "".to_string();

    let mut file_found: bool = false;

    for _i in 0..files {
        file_size = reader.read_u64();
        file_offset = reader.read_u64();
        file_name = reader.read_string();

        if Path::new(&file_name) == Path::new(asset_path) {
            file_found = true;
            break;
        }
    }

    if !file_found {
        println!("Failed to find file, {}", asset_path);
    }

    let file_dat = fs::File::open("data.dat");
    if !file_dat.is_ok() {
        println!("Failed to open dat file");
    }
    let mut file_dat = file_dat.unwrap();

    file_dat.seek(SeekFrom::Start(file_offset)).unwrap();

    let mut buffer: Vec<u8> = vec![0; file_size as usize];

    file_dat.read(&mut buffer).unwrap();

    println!(
        "Loaded asset: {}, Size: {}, Offset: {}",
        file_name, file_size, file_offset
    );

    buffer
}
