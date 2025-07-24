use super::*;

pub struct NumberDrawer;

impl NumberDrawer {
    // TODO: I'm well aware of the state of this section. I will refactor soon.
    pub fn draw(number: &str) -> String {
        let mut big_numbers: Vec<Vec<String>> = Vec::new();

        for digit in number.chars() {
            let big_number = String::from(Self::enlarge(&digit));
            let mut first_digit: Vec<String> = Vec::new();
            for line in big_number.lines() {
                first_digit.push(String::from(line))
            }
            big_numbers.push(first_digit);
        }

        let mut result = String::new();
        let height = big_numbers.get(0).unwrap().len();
        for i in 0..height {
            let first = big_numbers.get(0).unwrap();
            let second = big_numbers.get(1).unwrap();
            result.push_str(first.get(i).unwrap());
            result.push_str(second.get(i).unwrap());
            result.push_str("\n");
        }

        result
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
