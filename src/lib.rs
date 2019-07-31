use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

/// Likh - library to read and write to text and CSV files.


pub fn read_from_file(filename: &str) -> String{
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}

pub fn write_to_file(filename: &str, content: &str){

    fs::write(filename, content).expect("Cannot open file!");
}

pub fn read_from_csv<'a> (content: &'a str) -> Vec<Vec<&'a str>> {

    let mut data = vec![];

    for line in content.split("\n"){
        let mut line_vec = vec![];
        for row in line.split(","){
            line_vec.push(row);
        }
        data.push(line_vec);
    }
    return data;
}

pub fn write_to_csv<'a> (content: Vec<Vec<&str>>,file_path: &str) {
    let mut file = OpenOptions::new().append(true).create(true).open(file_path).unwrap();
    for row in content{
        let row_content = row.join(",");
        write!(&mut file, "{}\n",row_content).expect("Unable t owrite to file!")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn read_test() {
        let output = super::read_from_file("tests/data.txt");
        assert_eq!(output, "Hello, World!\n");
    }

    #[test]
    fn write_test() {
        super::write_to_file("tests/writetome.txt", "Test123#123#");
        let read_output = super::read_from_file("tests/writetome.txt");
        assert_eq!(read_output, "Test123#123#");
    }
 }