pub struct NumberDrawer;

impl NumberDrawer {
    // TODO: Refactor: WIP
    pub fn draw(number: &str) -> String {
        // TODO Use reference here and get rid of clone on line 34.
        let mut big_numbers: Vec<Vec<String>> = Vec::new();

        for digit in number.chars() {
            let big_number = String::from(Self::enlarge(&digit));
            let mut digit: Vec<String> = Vec::new();
            for line in big_number.lines() {
                digit.push(String::from(line))
            }
            big_numbers.push(digit);
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
            _ => String::from("NONE"),
        }
    }
}
