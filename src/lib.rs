use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

/// Likh - simple library to read and write to text and CSV files.

pub fn read_from_file(filename: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(filename) {
        Ok(c) => return Ok(String::from(c)),
        Err(why) => return Err(why),
    };
}

pub fn write_to_file(filename: &str, content: &str) -> Result<bool, std::io::Error> {
    match fs::write(filename, content) {
        Ok(_) => return Ok(true),
        Err(why) => return Err(why),
    }
}

pub fn read_from_csv(content: String) -> Vec<Vec<String>> {
    let mut data = vec![];


    for line in content.as_str().split("\n") {
        let mut line_vec = vec![];
        for row in line.split(",") {
            line_vec.push(String::from(row));
        }
        data.push(line_vec);
    }
    return data;
}

pub fn write_to_csv(content: Vec<Vec<String>>, file_path: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();
    for row in content {
        let row_content = row.join(",");
        write!(&mut file, "{}\n", row_content).expect("Unable to write to file!")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    

    #[test]
    fn read_write_csv_test() {
        let content = vec![vec![String::from("one"), String::from("two"), String::from("three"), String::from("four")]];
        super::write_to_csv(content, "target/test.csv");
        let content = std::fs::read_to_string("target/test.csv").unwrap();
        let csv = super::read_from_csv(content);

        assert_eq!(csv[0][0], String::from("one"));
    }
}

