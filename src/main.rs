use std::fs::File;
use std::io::Write;
use std::{fs, io};

const NGL_DIR: &str = "/Users/dbrotsky/Downloads/NGL";
const OUT_FILE: &str = "/Users/dbrotsky/Downloads/NGL-list.csv";

fn main() {
    let metadata = fs::metadata(NGL_DIR).unwrap();
    println!("Directory length is: {}MB", metadata.len() / (1024 * 1024));
    let mut buf = File::create(OUT_FILE).unwrap();
    writeln!(buf, "Index,Filename,Size (KB)").unwrap();
    let mut count = 0;
    for result in fs::read_dir(NGL_DIR).unwrap() {
        let file = result.unwrap();
        writeln!(
            &mut buf,
            "{},{},{}",
            count,
            file.file_name().to_str().unwrap(),
            file.metadata().unwrap().len() / 1024,
        )
        .unwrap();
        count += 1;
        if count % 100 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
        if count % 8_000 == 0 {
            println!(" ({})", count)
        }
    }
    if count % 8000 != 0 {
        println!(" ({})", count)
    }
}
