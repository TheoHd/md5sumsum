use std::env;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

fn main(){
    let mut p: Vec<String> = vec![];
    for arg in env::args().skip(1) {
        for e in WalkDir::new(arg).into_iter().filter_map(Result::ok).filter(|e| !e.file_type().is_dir()) {
            p.push(format!("{}", e.path().display()));
        }
    }

    let mut hashcat = String::from("");
    p.sort();
    for arg in p {
        let mut output = Command::new("md5sum")
                    .arg(arg)
                    .output()
                    .expect("failed to execute process");

        for e in String::from_utf8_lossy(&output.stdout).to_string().split_whitespace().next(){
            hashcat = format!("{}{}", hashcat, e.to_string());
        }
    }
    
    let mut output = Command::new("echo")
                    .arg(hashcat)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");

    let mut output2 = Command::new("md5sum")
                    .stdin(output.stdout.unwrap())
                    .output()
                    .expect("failed to execute process");

    let mut output3 = String::from_utf8_lossy(&output2.stdout).to_string();
    
    println!("{}", output3);
}