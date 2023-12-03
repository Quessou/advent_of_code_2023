fn find_next_number(s: &str, start_index: usize) -> Option<usize> {
    let next_char = s[start_index as usize..]
        .chars()
        .enumerate()
        .find(|c| c.1.is_digit(10));
    match next_char {
        Some((i, _)) => Some(start_index + i),
        None => None,
    }
}

fn find_end_of_number(s: &str, start_index: usize) -> usize {
    match s[start_index..]
        .chars()
        .enumerate()
        .find(|(_, c)| !c.is_digit(10))
    {
        Some((i, _)) => start_index + i,
        None => s.len(),
    }
}

fn build_number(s: &str) -> u32 {
    s.parse().unwrap()
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn is_close_to_symbol(
    current_line: &str,
    previous_line: &Option<&str>,
    next_line: &Option<&str>,
    mut start_index: usize,
    mut end_index: usize,
) -> bool {
    let mut chars_to_check: Vec<&str> = vec![];
    if start_index != 0 {
        chars_to_check.push(&current_line[start_index - 1..start_index]);
    } else {
        // kinda ugly tbh
        start_index = 1;
    }
    if end_index != current_line.len() {
        chars_to_check.push(&current_line[end_index..end_index + 1]);
    } else {
        // Kinda ugly too
        end_index = end_index - 1;
    }
    if let Some(p) = previous_line {
        chars_to_check.push(&p[start_index - 1..end_index + 1])
    }
    if let Some(p) = next_line {
        chars_to_check.push(&p[start_index - 1..end_index + 1])
    }

    chars_to_check
        .iter()
        .flat_map(|s| s.chars())
        .any(|c| is_symbol(c))
}

pub fn handle_line(
    current_line: &str,
    previous_line: &Option<&str>,
    next_line: &Option<&str>,
) -> Vec<u32> {
    let mut numbers = vec![];
    let mut start_index: usize = 0;
    while let Some(next_number_index) = find_next_number(current_line, start_index) {
        let end_of_number_index = find_end_of_number(current_line, next_number_index);
        let number = &current_line[next_number_index..end_of_number_index];
        //let number_len = end_of_number_index - next_number_index;

        // TODO : check for symbols around the number
        if is_close_to_symbol(
            current_line,
            previous_line,
            next_line,
            next_number_index,
            end_of_number_index,
        ) {
            let number = build_number(number);
            numbers.push(number);
        }

        start_index = end_of_number_index;
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;
    //#[test_case("help li" => "help".to_owned(); "Rather regular case" )]

    #[test_case("..23", 0 => Some(2))]
    #[test_case("..23..", 4 => None)]
    #[test_case("..23....2947", 5 => Some(8))]
    #[test_case("23.....", 0 => Some(0))]
    fn test_find_next_number(s: &str, start_index: usize) -> Option<usize> {
        find_next_number(s, start_index)
    }

    #[test_case("..23", 2 => 4)]
    #[test_case("..2319..", 2 => 6)]
    fn test_find_end_of_number(s: &str, start_index: usize) -> usize {
        find_end_of_number(s, start_index)
    }

    #[test_case("..27.....", Some(".*......."), None, 2, 4 => true; "Up Diagonal symbol on the left")]
    #[test_case("..27.....", Some("....*...."), None, 2, 4 => true; "Up Diagonal symbol on the right")]
    #[test_case("..27.....", Some("*........"), None, 2, 4 => false; "Up Diagonal symbol on the left but too far")]
    #[test_case("..27.....", Some("../......."), None, 2, 4 => true; "Up symbol")]
    #[test_case("..27.....", Some(".........."), Some("...*......"), 2, 4 => true; "Down symbol")]
    #[test_case("..27.....", Some(".........."), Some(".*........"), 2, 4 => true; "Down diagonal symbol on the left")]
    #[test_case("..27.....", Some(".........."), Some("....*....."), 2, 4 => true; "Down diagonal symbol on the right")]
    #[test_case(".$27.....", Some("........."), Some("........."), 2, 4 => true; "Symbol to the left")]
    #[test_case("..27+....", Some("........."), Some("........."), 3, 5 => true; "Symbol to the right")]
    #[test_case("..27.....", Some(".........."), Some(".........."), 2, 4 => false; "No symbol")]
    #[test_case("271.......", Some(".........."), Some("...*......"), 0, 3 => true; "Down diagonal at beginning")]
    fn test_is_close_to_symbol(
        cur: &str,
        prev: Option<&str>,
        next: Option<&str>,
        start_index: usize,
        end_index: usize,
    ) -> bool {
        is_close_to_symbol(cur, &prev, &next, start_index, end_index)
    }

    #[ignore = "Not working yet"]
    #[test_case(()=> ())]
    fn test_handle_first_line(_: ()) {
        let current_line = "467..114..";
        let previous_line: Option<&str> = None;
        let next_line = Some("...*......");
        let numbers = handle_line(current_line, &previous_line, &next_line);
        assert_eq!(numbers.len(), 1);
        assert_eq!(numbers[0], 467);
    }
}
