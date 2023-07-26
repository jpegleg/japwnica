use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn inspect(file_path: &str) {
    let file_path = Path::new(file_path);
    let mut file = File::open(&file_path).expect("Failed to open the file.");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to read the file");
    print!("{{ {:?} }};\n", bytes);
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = args[1].clone();
    let sdata = String::from(data);
    inspect(&sdata);
}
