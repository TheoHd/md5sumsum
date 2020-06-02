#![feature(test)]

use std::io;
use std::thread;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::env;
use walkdir::{WalkDir, DirEntry as WalkDirEntry};
use md5;
use rayon::prelude::*;
use crypto::md5::Md5;
use crypto::digest::Digest;
use jwalk::WalkDir as JWalkDir;
use ignore::Walk;

extern crate test;

pub fn walkdir_in_args() {
    for arg in env::args().skip(1) {
        for _x in WalkDir::new(arg).into_iter().filter_map(|e| e.ok()) {
        }
    }
}

pub fn walkdir_with_rayon() {
    for arg in env::args().skip(1) {
        WalkDir::new(arg).into_iter().filter_map(|e| e.ok()).collect::<Vec<WalkDirEntry>>().par_iter();
    }
}

pub fn get_hashcat() -> String {
    let mut hashcat = String::new();
    let mut digest;
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            digest = md5::compute(String::from(x.path().to_string_lossy()));
            hashcat += format!("{:x}",digest).to_string().split(" ").collect::<Vec<&str>>()[0];
        }
    }
    hashcat
}

pub fn get_hashcat_crypt() -> String {
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


fn get_hashcat_crypt_2() -> Vec<u8>{
    let mut hashcat: Vec<[u8; 16]> = Vec::new();
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            let mut digest = Md5::new();
            let mut output = [0; 16];
            digest.input_str(x.path().to_str().unwrap());
            digest.result(&mut output);
            //println!("Result:{:?}", output);
            hashcat.push(output)
            //hashcat += digest.result_str().to_string().split(" ").collect::<Vec<&str>>()[0];
        }
    }
    hashcat.concat()
}

fn get_hashcat_crypt_3() -> String {
    let mut hashcat: String = String::new();
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            let mut digest = Md5::new();
            digest.input_str(x.path().to_str().unwrap());
            hashcat += &digest.result_str();
        }
    }
    hashcat
}

fn get_hashcat_crypt_4() -> String {
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

fn get_hashcat_crypt_3_mt() -> String {
    let mut hashcat: Vec<String> = Vec::new();
    let handle = thread::spawn(move|| {
        for arg in env::args().skip(1) {
                for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
                    let mut digest = Md5::new();
                    digest.input_str(x.path().to_str().unwrap());
                    // hashcat += &digest.result_str();
                    println!("{}", digest.result_str());
                    hashcat.push(digest.result_str());
                }
        }
    });
    handle.join().unwrap();
    String::from("")
}

fn get_hashcat_crypt_5() -> String {
    let mut paths: Vec<WalkDirEntry> = Vec::new();
    let mut i = 0;
    for arg in env::args().skip(1){
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
            paths.push(x);
        }
    }
    paths.par_iter().map(|x| {
        let mut digest = Md5::new();
        digest.input_str(x.path().to_str().unwrap());
        let temp = digest.result_str();
        drop(digest);
        temp
    }).collect::<Vec<String>>().concat()
}

fn get_hashcat_crypt_6() -> String {
    let mut paths = Vec::new();
    let mut i = 0;
    for arg in env::args().skip(1){
        for x in JWalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
            paths.push(x);
        }
    }
    paths.par_iter().map(|x| {
        let mut digest = Md5::new();
        digest.input_str(x.path().to_str().unwrap());
        let temp = digest.result_str();
        drop(digest);
        temp
    }).collect::<Vec<String>>().concat()
}

fn get_hashcat_crypt_7() -> String {
    let mut paths = Vec::new();
    let mut i = 0;
    for arg in env::args().skip(1){
        for x in Walk::new(arg).into_iter().filter_map(Result::ok) {
            paths.push(x);
        }
    }
    paths.par_iter().map(|x| {
        let mut digest = Md5::new();
        digest.input_str(x.path().to_str().unwrap());
        let temp = digest.result_str();
        drop(digest);
        temp
    }).collect::<Vec<String>>().concat()
}

fn args_loop() {
    for arg in env::args().skip(1){

    }
}

fn args_loop_2() {
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args{

    }
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

pub fn final_print_crypt(hashcat: &mut String) {
    let mut digest = Md5::new();
    digest.input_str(hashcat);
    let mut digest2 = Md5::new();
    digest2.input_str(&digest.result_str());
    drop(digest);
    print!("{}", digest2.result_str());
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
    fn test_get_hashcat_crypt(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt());
    }

    #[bench]
    fn test_get_hashcat_crypt_2(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt_2());
    }

    #[bench]
    fn test_get_hashcat_crypt_3(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt_3());
    }

    #[bench]
    fn test_get_hashcat_crypt_3_mt(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt_3_mt());
    }

    #[bench]
    fn test_get_hashcat_crypt_4(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt_4());
    }

    #[bench]
    fn test_get_hashcat_crypt_5(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt_5());
    }

    
    #[bench]
    fn test_get_hashcat_crypt_6(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt_6());
    }
    #[bench]
    fn test_get_hashcat_crypt_7(b: &mut Bencher){
        b.iter(|| get_hashcat_crypt_7());
    }

    #[bench]
    fn test_final_print(b: &mut Bencher){
        let mut hashcat = get_hashcat();
        b.iter(|| final_print(&mut hashcat));
    }

    #[bench]
    fn test_final_print_crypt(b: &mut Bencher){
        let mut hashcat = get_hashcat();
        b.iter(|| final_print_crypt(&mut hashcat));
    }

    #[bench]
    fn test_walkdir_with_rayon(b: &mut Bencher){
        b.iter(|| walkdir_with_rayon());
    }

    #[bench]
    fn test_args_loop(b: &mut Bencher){
        b.iter(|| args_loop());
    }
    #[bench]
    fn test_args_loop_2(b: &mut Bencher){
        b.iter(|| args_loop_2());
    }
}