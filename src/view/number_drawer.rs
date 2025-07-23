use super::*;

pub struct NumberDrawer;

impl NumberDrawer {
    pub fn draw(number: &str) -> String {
        let mut big_numbers: Vec<Vec<String>> = number.chars();

        for digit in number.chars() {
            if digit.is_ascii_digit() {
                let big_number = String::from(Self::enlarge(&digit).trim());
                big_numbers.push_str(&big_number);
            }
        }
        big_numbers
    }

    fn enlarge(digit: &char) -> String {
        match digit {
            '0' => String::from(
                "
╔═══╗
║╔═╗║
║║ ║║
║║ ║║
║╚═╝║
╚═══╝
",
            ),

            '1' => String::from(
                "
 ╔╗ 
╔╝║ 
╚╗║ 
 ║║ 
╔╝╚╗
╚══╝
",
            ),
            '2' => String::from(
                "
╔═══╗
║╔═╗║
╚╝╔╝║
╔═╝╔╝
║║╚═╗
╚═══╝
",
            ),
            '3' => String::from(
                "
╔═══╗
║╔═╗║
╚╝╔╝║
╔╗╚╗║
║╚═╝║
╚═══╝
",
            ),
            '4' => String::from(
                "
╔╗ ╔╗
║║ ║║
║╚═╝║
╚══╗║
   ║║
   ╚╝

",
            ),
            '5' => String::from(
                "
╔═══╗
║╔══╝
║╚══╗
╚══╗║
╔══╝║
╚═══╝

",
            ),
            '6' => String::from(
                "
╔═══╗
║╔══╝
║╚══╗
║╔═╗║
║╚═╝║
╚═══╝

",
            ),
            '7' => String::from(
                "
╔═══╗
║╔═╗║
╚╝╔╝║
  ║╔╝
  ║║ 
  ╚╝ 

",
            ),
            '8' => String::from(
                "
╔═══╗
║╔═╗║
║╚═╝║
║╔═╗║
║╚═╝║
╚═══╝

",
            ),
            '9' => String::from(
                "
╔═══╗
║╔═╗║
║╚═╝║
╚══╗║
╔══╝║
╚═══╝

",
            ),
            _ => String::from("NONE"),
        }
    }
}
