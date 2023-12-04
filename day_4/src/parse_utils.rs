pub fn parse_list_of_ints(s: &str, separator: char) -> Vec<u32> {
    s.trim()
        .split(separator)
        .filter(|s| !s.is_empty())
        .map(|n| n.trim().parse().unwrap())
        .collect()
}
