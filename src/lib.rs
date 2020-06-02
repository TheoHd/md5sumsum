#![feature(test)]

use std::env;
use walkdir::WalkDir;
use crypto::md5::Md5;
use crypto::digest::Digest;


extern crate test;

pub fn walkdir_in_args() {
    for arg in env::args().skip(1) {
        for _x in WalkDir::new(arg).into_iter().filter_map(|e| e.ok()) {
        }
    }
}

pub fn get_hashcat() -> String{
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
    hashcat
}

pub fn echo_cmd(hashcat: &mut String) {
    let mut digest = Md5::new();
    digest.input_str(hashcat);
}

pub fn final_print(hashcat: &mut String) {
    let mut digest = Md5::new();
    let mut digest2 = Md5::new();
    digest2.input_str(hashcat);
    digest.input_str(&digest2.result_str());
    print!("{}",
        digest.result_str()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn test_walkdir_in_args(b: &mut Bencher){
        b.iter(|| walkdir_in_args());
    }
    
    #[bench]
    fn test_echo_cmd(b: &mut Bencher){
        let mut hashcat = get_hashcat();
        b.iter(|| echo_cmd(&mut hashcat));
    }

    #[bench]
    fn test_get_hashcat(b: &mut Bencher){
        b.iter(|| get_hashcat());
    }

    #[bench]
    fn test_final_print(b: &mut Bencher){
        let mut hashcat = get_hashcat();
        b.iter(|| final_print(&mut hashcat));
    }
}