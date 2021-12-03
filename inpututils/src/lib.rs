use std::str::FromStr;

pub fn read_all(file_name: &str) -> Vec<String> {
    std::fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("file not found: {}", file_name))
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn read_all_as<T: FromStr>(file_name: &str) -> Vec<T> {
    read_all(file_name)
        .iter()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
