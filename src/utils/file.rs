use std::fs::OpenOptions;
use std::io::{self, Write};

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
