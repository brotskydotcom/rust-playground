use chrono::{DateTime, SecondsFormat, Utc};
use std::fs::File;
use std::io::Write;
use std::{fs, io};

const NGL_DIR: &str = "/Users/dbrotsky/tmp/NGL";
const OUT_FILE: &str = "/Users/dbrotsky/tmp/NGL-list.csv";

fn main() {
    let metadata = fs::metadata(NGL_DIR).unwrap();
    println!("Directory length is: {}MB", metadata.len() / (1024 * 1024));
    let mut buf = File::create(OUT_FILE).unwrap();
    writeln!(buf, "Index,Filename,Size,ModDate").unwrap();
    let mut count = 0;
    for result in fs::read_dir(NGL_DIR).unwrap() {
        let file = result.unwrap();
        let name = file.file_name().to_string_lossy().to_string();
        let meta = file.metadata().unwrap();
        let mod_time: DateTime<Utc> = meta.modified().unwrap().into();
        let mod_time = mod_time.to_rfc3339_opts(SecondsFormat::Secs, true);
        writeln!(&mut buf, "{},{},{},{}", count, name, meta.len(), mod_time).unwrap();
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
