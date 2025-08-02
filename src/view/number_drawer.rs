pub struct NumberDrawer;

impl NumberDrawer {
    pub fn draw(number: &str) -> String {
        // TODO: Could use &str instead and get rid off digit_line[index].clone().
        let mut big_numbers: Vec<Vec<String>> = Vec::new();

        for digit in number.chars() {
            let big_digit = Self::enlarge(&digit);
            let digit_line: Vec<String> = big_digit.lines().map(|line| line.to_string()).collect();
            big_numbers.push(digit_line);

            if !digit.is_ascii_digit() {
                break
            }
        }

        let height = big_numbers[0].len();

        (0..height)
            .map(|index| {
                big_numbers
                    .iter()
                    .map(|digit_line| digit_line[index].clone())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
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
            _ => String::from(
                "
  ,_,  
 (.,.) 
 (   ) 
--\"-\"---
",
            ),
        }
    }
}
