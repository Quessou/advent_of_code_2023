use std::any;

pub struct CalibrationValue(pub u32);

static RADIX: u32 = 10;
static NUMBERS: &'static [&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn string_number_to_digit(data: &str) -> Result<u32, ()> {
    match data {
        "one" => Ok(1),
        "two" => Ok(2),
        "three" => Ok(3),
        "four" => Ok(4),
        "five" => Ok(5),
        "six" => Ok(6),
        "seven" => Ok(7),
        "eight" => Ok(8),
        "nine" => Ok(9),
        _ => unreachable!(),
    }
}

fn find_first(data: &str) -> u32 {
    let toto = data
        .char_indices()
        .find(|(i, c)| c.is_digit(RADIX) || NUMBERS.iter().any(|n| data[*i..].starts_with(n)));
    assert!(toto.is_some());
    // God this is ugly
    if toto.unwrap().1.is_digit(RADIX) {
        return String::from(toto.unwrap().1).parse().unwrap();
    }
    let number = NUMBERS
        .iter()
        .find_map(|n| {
            if data[toto.unwrap().0..].starts_with(n) {
                Some(n)
            } else {
                None
            }
        })
        .unwrap();
    string_number_to_digit(number).unwrap()
}

fn find_last(data: &str) -> u32 {
    let toto = data
        .char_indices()
        .rev()
        .find(|(i, c)| c.is_digit(RADIX) || NUMBERS.iter().any(|n| data[*i..].starts_with(n)));
    assert!(toto.is_some());
    // God this is ugly
    if toto.unwrap().1.is_digit(RADIX) {
        return String::from(toto.unwrap().1).parse().unwrap();
    }
    let number = NUMBERS
        .iter()
        .find_map(|n| {
            if data[toto.unwrap().0..].starts_with(n) {
                Some(n)
            } else {
                None
            }
        })
        .unwrap();
    string_number_to_digit(number).unwrap()
}

impl From<&str> for CalibrationValue {
    fn from(value: &str) -> Self {
        let radix = 10;
        let first_digit: u32 = find_first(value);
        let last_digit: u32 = find_last(value);
        CalibrationValue(first_digit * radix + last_digit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_calibration_instanciation() {
        let s = "abdjg3ir2ijgb7ezig";
        assert_eq!(CalibrationValue::from(s).0, 37);
    }

    #[test]
    fn test_one_digit_case_instanciation() {
        let s = "abdjgir2ijgbezig";
        assert_eq!(CalibrationValue::from(s).0, 22);
    }

    #[test]
    fn test_find_first() {
        let s = "jejione3idjeiijr7";
        assert_eq!(find_first(s), 1);
        let s = "je2jione3idjeiijr7";
        assert_eq!(find_first(s), 2);
        let s = "seventwoje2jione3idjeiijr7";
        assert_eq!(find_first(s), 7);
    }
    #[test]
    fn test_find_last() {
        let s = "jejione3idjeiijr7";
        assert_eq!(find_last(s), 7);
        let s = "je2jione3idjeiijr7eight";
        assert_eq!(find_last(s), 8);
        let s = "seventwojejioneidjeiijr";
        assert_eq!(find_last(s), 1);
    }
}
