use std::env;
use walkdir::WalkDir;
use md5;

fn get_hashcat() -> String{
    let mut hashcat = String::from("");
    let mut digest;
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            digest = md5::compute(String::from(x.path().to_string_lossy()));
            hashcat += format!("{:x}",digest).to_string().split(" ").collect::<Vec<&str>>()[0];
        }
    }
    hashcat
}

fn main(){
    let hashcat = get_hashcat();
    print!("{}",format!("{:x}",md5::compute(format!("{:x}",md5::compute(hashcat)))));
} 