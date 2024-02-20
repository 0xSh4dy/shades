use std::{fs::File, io::Read};

pub fn open_file(path:&String)->Result<File,std::io::Error>{
    let file = File::open(path);
    file
}

pub fn read_file_string(mut file:File)->Result<String,Box<dyn std::error::Error>>{
    let mut buf = String::new();
    File::read_to_string(&mut file, &mut buf)?;
    Ok(buf)
}