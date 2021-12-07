use std::str::FromStr;

pub fn read_lines(file_name: &str) -> Vec<String> {
    std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("file not found: {}", file_name))
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn read_lines_as<T: FromStr>(file_name: &str) -> Vec<T> {
    std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("file not found: {}", file_name))
        .lines()
        .map(|line| match line.parse::<T>() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse"),
        })
        .collect()
}

pub fn read_comma_separated_as<T: FromStr>(file_name: &str) -> Vec<T> {
    std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("file not found: {}", file_name))
        .split(',')
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse"),
        })
        .collect()
}

// Reference based methods. The called holds the file string & everything is borrowed from there.

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
}

pub fn lines(string: &String) -> impl Iterator<Item = &str> {
    string.lines()
}