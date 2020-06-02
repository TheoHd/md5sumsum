#![feature(test)]

use std::env;
use std::process::{Command, Stdio};
use walkdir::WalkDir;
use md5;

extern crate test;

pub fn walkdir_in_args() {
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
        }
    }
}

pub fn md5sum_with_no_hashcat() {
    let mut output;
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            output = Command::new("md5sum")
            .arg(x.path().display().to_string())
            .output()
            .expect("failed to execute process");
        }
    }
}

pub fn md5sum_with_hashcat() {
    let mut hashcat = String::from("");
    let mut output;
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            output = Command::new("md5sum")
            .arg(x.path().display().to_string())
            .output()
            .expect("failed to execute process");
            for e in String::from_utf8_lossy(&output.stdout).to_string().split_whitespace().next(){
                hashcat = format!("{}{}", hashcat, e.to_string());
            }
        }
    }
}

pub fn get_hashcat() -> String{
    let mut hashcat = String::from("");
    //let mut output;
    let mut digest;
    for arg in env::args().skip(1) {
        for x in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            digest = md5::compute(String::from(x.path().to_string_lossy()));
            //output = Command::new("md5sum")
            //    .arg(String::from(x.path().to_string_lossy()))
            //    .output()
            //    .unwrap();
            //hashcat += String::from_utf8_lossy(&output.stdout).to_string().split(" ").collect::<Vec<&str>>()[0];
            hashcat += format!("{:x}",digest).to_string().split(" ").collect::<Vec<&str>>()[0];
        }
    }
    hashcat
}

pub fn echo_cmd(hashcat: &mut String){
    md5::compute(hashcat);
    //Command::new("md5sum")
    //      .arg(hashcat)
            //.output()
            //.unwrap();
}

pub fn final_print(hashcat: &mut String) {
    print!("{}",
        format!("{:x}",md5::compute(format!("{:x}",md5::compute(hashcat))))
        //String::from_utf8_lossy(
            //&Command::new("md5sum")
            //.stdin(
                //Command::new("echo")
                //.arg(hashcat)
                //.stdout(Stdio::piped())
                //.spawn()
                //.expect("failed to execute process")
                //.stdout.unwrap()
            //)
            //.output()
            //.expect("failed to execute process").stdout
        //).to_string()
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
    fn test_md5sum_with_no_hashcat(b: &mut Bencher){
        b.iter(|| md5sum_with_no_hashcat());
    }

    #[bench]
    fn test_md5sum_with_hashcat(b: &mut Bencher){
        b.iter(|| md5sum_with_hashcat());
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