use std::fs::OpenOptions;
use std::io::{self, Write};
use std::string;

// ファイルを開いて、存在しなければ新規作成する関数
pub fn open_or_create_file(file_name: &str) -> io::BufWriter<std::fs::File> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_name)
        .expect("Failed to open or create file");
    io::BufWriter::new(file)
}

pub fn load_txt(file_name: &str) {
    println!("load text {}", file_name);
}

pub fn write_txt(file_name: &str){
    println!("write text {}", file_name);
}

pub fn create_txt(file_name: &str){
    println!("create text {}", file_name);
}
