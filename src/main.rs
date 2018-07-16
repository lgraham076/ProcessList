
extern crate glob;

fn main() {
    use glob::glob;

    // Output all nodes in /proc
    for entry in glob("/proc/*").expect("Unable to read directory") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        } 
    }
}