
use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate glob;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    parse_proc();
}

fn parse_proc() {
    use glob::glob;
    let mut pid_stat = vec![];

    // Output all numbered nodes in /proc
    for entry in glob("/proc/[0-9]*").expect("Unable to read directory") {
        match entry {
            //Ok(path) => println!("{:?}", path.display()),
            Ok(path) => pid_stat.push(read_line(format!("{}", path.display()))),
            Err(e) => println!("{:?}", e),
        } 
    }

    println!("{:?}", pid_stat);
}

fn read_line(stat_folder: String) -> String {
    let stat_path = format!("{}/{}", stat_folder, "stat");
    //println!("{}", stat_path);

    let mut stat_file = File::open(stat_path).expect("File not found");
    let mut line = String::new();
    stat_file.read_to_string(&mut line).expect("Unable to read file");

    line
}