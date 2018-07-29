
use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate glob;

// Main
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    parse_proc();
}

// Read each stat file from the /proc/<pid>
fn parse_proc() {
    use glob::glob;
    let mut pid_stat = vec![];

    // Output all numbered nodes in /proc
    for entry in glob("/proc/[0-9]*").expect("Unable to read directory") {
        // Read stat file for all entries matching /proc/<pid> format
        match entry {
            // Parse stat file
            Ok(path) => pid_stat.push(read_line(format!("{}", path.display()))),
            // Print error if found
            Err(e) => println!("{:?}", e),
        } 
    }
    // Print all entries found in vector
    println!("{:?}", pid_stat);
}

// Read stat file and return the information within
fn read_line(stat_folder: String) -> String {
    let stat_path = format!("{}/{}", stat_folder, "stat");
    // Open file
    let mut stat_file = File::open(stat_path).expect("File not found");
    // Read contents into string
    let mut line = String::new();
    stat_file.read_to_string(&mut line).expect("Unable to read file");

    // Return string
    line
}