use std::fs;
use std::fs::File;
use std::io::Write;

const NGL_DIR: &str = "/Users/dbrotsky/Library/Logs/NGL";
const OUT_FILE: &str = "/tmp/ngl-list.txt";

fn main() {
    let result = fs::metadata(NGL_DIR).unwrap();
    println!("Directory length is: {}MB", result.len() / (1024 * 1024));
    let mut buf = File::create(OUT_FILE).unwrap();
    for (i, file) in fs::read_dir(NGL_DIR).unwrap().enumerate() {
        let file = file.unwrap();
        writeln!(
            &mut buf,
            "{}: {} ({}MB)",
            i,
            file.file_name().to_str().unwrap(),
            file.metadata().unwrap().len(),
        )
        .unwrap();
        if i > 0 && i % 100 == 0 {
            print!(".")
        }
        if i % 8_000 == 0 {
            println!("{}: ", i)
        }
    }
}
