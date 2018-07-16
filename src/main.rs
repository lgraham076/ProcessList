
extern crate glob;

fn main() {
    parse_proc();
}

fn parse_proc() {
    use glob::glob;

    // Output all numbered nodes in /proc
    for entry in glob("/proc/[0-9]*").expect("Unable to read directory") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        } 
    }
}