use clap::Parser;
use std::fs::{metadata, File};
use std::io::Write;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::str;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of lines to read
    #[arg(short, long, default_value_t = 10)]
    number: u32,

    /// The filename to read
    filename: Option<String>,
}

fn main() {
    let args = Args::parse();
    let line_count: u32 = args.number + 1;
    let filename = args.filename.as_deref().expect("No filename provided");

    let chunk: i64 = (1 << 10) * 4; // 4kb

    let path = Path::new(filename);
    let f = File::open(path).expect("Unable to open file");
    let md = metadata(path).expect("Unable to get file metadata");
    let len = md.len() as i64;

    let mut pos = len;
    let mut found_newlines = 0;

    while pos > 0 && found_newlines < line_count {
        pos = if pos - chunk < 0 { 0 } else { pos - chunk };
        let read_len = if pos + chunk < len { len - pos } else { chunk };
        let mut buffer = vec![0; read_len as usize];
        let _read_size = f.read_at(&mut buffer, pos as u64).expect("Failed to read");
        for i in (0.._read_size).rev() {
            if '\n' == char::from_u32(buffer[i] as u32).unwrap() {
                found_newlines += 1
            }
            if found_newlines == line_count {
                pos = pos + i as i64 + 1;
                break;
            }
        }
    }

    while pos < len {
        let read_len = if pos + chunk < len { len - pos } else { chunk };
        let mut buffer = vec![0; read_len as usize];
        let read_size = f.read_at(&mut buffer, pos as u64).expect("Failed to read");
        pos += read_size as i64;
        std::io::stdout()
            .write(&buffer[0..read_size])
            .expect("failed to write to stdout");
    }
}
