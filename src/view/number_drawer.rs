use super::*;

pub struct NumberDrawer;

impl NumberDrawer {
    pub fn draw(number: &str) -> String {
        let mut big_numbers = String::new();
        for digit in number.chars() {
            if digit.is_ascii_digit() {
                big_numbers.push_str(&Self::enlarge(digit));
                big_numbers.push_str(" ");
            }
        }
        big_numbers
    }

    fn enlarge(digit: char) -> String {
        match digit {
            '0' => String::from("ZERO"),
            '1' => String::from("ONE"),  
            '2' => String::from("TWO"),
            '3' => String::from("THREE"),
            '4' => String::from("FOUR"),
            '5' => String::from("FIVE"),
            '6' => String::from("SIX"),
            '7' => String::from("SEVEN"),
            '8' => String::from("EIGHT"),
            '9' => String::from("NINE"),
            _ => String::from("NONE"),
        }
    }
}
