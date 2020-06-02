#![feature(test)]

use std::env;
use walkdir::{WalkDir, DirEntry};
use md5;

extern crate test;

pub fn walkdir_in_args() {
    for arg in env::args().skip(1) {
        for _x in WalkDir::new(arg).into_iter().filter_map(|e| e.ok()) {
        }
    }
}

pub fn get_hashcat() -> String {
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

pub fn echo_cmd(hashcat: &mut String) {
    md5::compute(hashcat);
}

pub fn final_print(hashcat: &mut String) {
    print!(
        "{}",
        format!(
            "{:x}",
            md5::compute(
                format!(
                    "{:x}",
                    md5::compute(hashcat)
                )
            )
        )
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