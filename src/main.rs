use std::env;
use walkdir::WalkDir;
use md5;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::thread;

fn get_hashcat() -> String {
    let mut hashcat: String = String::new();
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg) {
            match x {
                Err(y) => continue,
                Ok(de) => {
                    let is_file = de.file_type().is_file();
                    if !is_file {
                        continue;    
                    }
                    let mut digest = Md5::new();
                    let filepath = de.path().to_str().unwrap();
                    digest.input_str(filepath);
                    hashcat += &digest.result_str();
                }
            };
        }
    }
    hashcat
}

fn main(){
    let hashcat = get_hashcat();
    let mut digest = Md5::new();
    digest.input_str(&hashcat);
    let mut digest2 = Md5::new();
    digest2.input_str(&digest.result_str());
    println!("{}", digest2.result_str());
} 