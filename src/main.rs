use std::env;
use walkdir::WalkDir;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main(){
    let mut hashcat = String::from("");

    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(|e| e.ok()) {
            if !x.file_type().is_file() {
                continue;
            }
            let mut digest = Md5::new();
            digest.input_str(&x.path().display().to_string());
            hashcat += digest.result_str().split(" ").collect::<Vec<&str>>()[0];
        }
    }

    let mut digest = Md5::new();
    let mut digest2 = Md5::new();
    digest.input_str(&hashcat);
    digest2.input_str(&digest.result_str());
    print!("{}\n",
        digest2.result_str()
    );
}