/*
#[derive(Default)]
pub struct Gear {
    pub numbers: Vec<u32>,
}

impl Gear {
    pub fn build_gear(
        current_line: &mut str,
        previous_line: &Option<&str>,
        next_line: &Option<&str>,
        index: usize,
    ) -> Gear {
        let mut numbers: Vec<u32> = vec![];
        unsafe {
            let mut line_as_bytes = current_line.as_bytes_mut();
            if index != 0 && line_as_bytes[index - 1].is_ascii_digit() {
                let mut back_offset = 1;
                while line_as_bytes[index - back_offset].is_ascii_digit() {
                    back_offset = back_offset + 1;
                }
                unsafe {
                    let number = String::from_raw_parts(
                        line_as_bytes[index - back_offset..index].as_mut_ptr(),
                        back_offset,
                        back_offset,
                    )
                    .to_owned()
                    .parse::<u32>()
                    .unwrap();
                    numbers.push(number);
                }
            }
            if index != current_line.len() && line_as_bytes[index + 1].is_ascii_digit() {
                let mut front_offset = 1;
                while line_as_bytes[index + front_offset].is_ascii_digit() {
                    front_offset = front_offset + 1;
                }
                unsafe {
                    let number = String::from_raw_parts(
                        line_as_bytes[index..index + front_offset].as_mut_ptr(),
                        front_offset,
                        front_offset,
                    )
                    .to_owned()
                    .parse::<u32>()
                    .unwrap();
                    numbers.push(number);
                }
            }
            if let Some(line) = previous_line {}

            if let Some(line) = next_line {}
            Gear::default()
        }
    }

    pub fn is_valid_gear(&self) -> bool {
        self.numbers.len() == 2
    }
}
*/
