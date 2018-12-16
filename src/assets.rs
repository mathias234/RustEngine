use bincode::{deserialize, deserialize_from, serialize};
use std::fs;
use std::fs::Metadata;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Cursor;
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
    if path_exists("./data.idx") {
        println!("deleting data.idx");
        fs::remove_file("./data.idx").unwrap();
    }

    if path_exists("./data.dat") {
        println!("deleting data.dat");
        fs::remove_file("./data.dat").unwrap();
    }

    let mut curr_data_loc: u32 = 0;
    let mut idx_buffer = fs::File::create("data.idx").unwrap();
    let mut dat_buffer = fs::File::create("data.dat").unwrap();

    let mut files: Vec<MetaFile> = Vec::new();

    recursive_search("./res", &mut files);

    write_u32(files.len() as u32, &mut idx_buffer);

    for file in files {
        println!(
            "Building file, Name: {}, Size: {}",
            file.file,
            file.metadata.len()
        );

        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata.len() as usize);
        // initialize the vec with data
        for _i in 0..file.metadata.len() {
            buffer.push(0);
        }

        let mut file_dat = fs::File::open(&file.file);

        if !file_dat.is_ok() {
            println!("Failed to open file");
        }

        let mut file_dat = file_dat.unwrap();

        file_dat.read(&mut buffer).unwrap();

        dat_buffer.write(&buffer).unwrap();

        // Write idx entry
        {
            // write file size
            write_u32(file.metadata.len() as u32, &mut idx_buffer);
            // write file offset
            write_u32(curr_data_loc, &mut idx_buffer);
            // write file name
            write_string(file.file, &mut idx_buffer);
        }

        curr_data_loc = curr_data_loc + file.metadata.len() as u32;
    }

    // test
    get_asset("./res/nicebrick_nrm.jpg");

    loop {}
}

pub fn get_asset(asset_path: &str) -> Vec<u8> {
    println!("Loading asset, {}", asset_path);

    // open idx and find file
    let file_idx = fs::File::open("data.idx");
    if !file_idx.is_ok() {
        println!("Failed to open idx file");
    }
    let mut file_idx = file_idx.unwrap();

    let files = read_u32(&mut file_idx);

    let mut file_size: u32 = 0;
    let mut file_offset: u32 = 0;
    let mut file_name: String = "".to_string();

    let mut file_found: bool = false;

    for _i in 0..files {
        file_size = read_u32(&mut file_idx);
        file_offset = read_u32(&mut file_idx);
        file_name = read_string(&mut file_idx);

        if Path::new(&file_name) == Path::new(asset_path) {
            file_found = true;
            break;
        }
    }

    if !file_found {
        println!("Failed to find file, {}", asset_path);
    }

    println!(
        "File: {}, Size: {}, Offset: {}",
        file_name, file_size, file_offset
    );

    let mut file_dat = fs::File::open("data.dat");
    if !file_dat.is_ok() {
        println!("Failed to open dat file");
    }
    let mut file_dat = file_dat.unwrap();

    file_dat.seek(SeekFrom::Start(file_offset as u64)).unwrap();

    let mut buffer: Vec<u8> = Vec::with_capacity(file_size as usize);
    // initialize the vec with data
    for i in 0..file_size {
        buffer.push(0);
    }

    file_dat.read(&mut buffer).unwrap();

    buffer
}

fn write_string(value: String, writer: &mut fs::File) {
    let data: Vec<u8> = serialize(&value).unwrap();
    writer.write(&data).unwrap();
}

fn write_u32(value: u32, writer: &mut fs::File) {
    let data: Vec<u8> = serialize(&value).unwrap();
    writer.write(&data).unwrap();
}

fn read_u32(reader: &mut fs::File) -> u32 {
    let mut buffer: Vec<u8> = Vec::new();

    // 4 bytes
    for _i in 0..4 {
        buffer.push(0);
    }

    reader.read(&mut buffer).unwrap();

    let value: u32 = deserialize(&buffer).unwrap();

    value
}

fn read_u64(reader: &mut fs::File) -> u64 {
    let mut buffer: Vec<u8> = Vec::new();

    // 4 bytes
    for _i in 0..8 {
        buffer.push(0);
    }

    reader.read(&mut buffer).unwrap();

    let value: u64 = deserialize(&buffer).unwrap();

    value
}

fn read_string(reader: &mut fs::File) -> String {
    deserialize_from(reader).unwrap()
}
