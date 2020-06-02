use std::env;
use walkdir::WalkDir;
use md5;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn get_hashcat() -> String{
    let mut hashcat = String::new();
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            let mut digest = Md5::new();
            digest.input_str(&String::from(x.path().to_string_lossy()));
            hashcat += digest.result_str().to_string().split(" ").collect::<Vec<&str>>()[0];
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