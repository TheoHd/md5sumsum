use std::env;
use walkdir::WalkDir;
use md5;

fn main(){
    let mut hashcat = String::new();
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
            hashcat += &format!("{:x}", md5::compute(x.path().display().to_string()));
        }
    }
    print!("{}",format!("{:x}",md5::compute(format!("{:x}",md5::compute(hashcat)))));
}