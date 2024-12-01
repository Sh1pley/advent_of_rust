pub mod file_parse {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    // borrowed fun. May be another implementation in Rust, but this seems safe and works.
    pub fn get_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
            //iterate through each line of input file
            let file = File::open(file_path)?;
            Ok(io::BufReader::new(file).lines())
    }
}
