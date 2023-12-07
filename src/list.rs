use std::fs;

pub fn list_files() {
    let paths: fs::ReadDir = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}
