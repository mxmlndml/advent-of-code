const EXAMPLE_SOLUTION: &str = "4361";

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u32,
    start: usize,
    end: usize,
    row: usize,
}
impl Number {
    fn new(row: usize) -> Number {
        Number {
            value: 0,
            start: 0,
            end: 0,
            row,
        }
    }
}

#[derive(Debug)]
struct Symbol {
    row: usize,
    pos: usize,
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut result = "Foo".to_string();
    let mut sum = 0;
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    input.lines().enumerate().for_each(|(row, text)| {
        let mut current_number = Number::new(row);
        let mut digit = 0;
        text.bytes()
            .enumerate()
            .rev()
            .map(|(i, byte)| (i, byte as char))
            .for_each(|(pos, character)| {
                if character.is_digit(10) {
                    if digit == 0 {
                        current_number.end = pos;
                    }
                    current_number.value +=
                        character.to_digit(10).expect("not a digit") * 10_u32.pow(digit);
                    digit += 1;
                    return;
                }

                if digit != 0 {
                    current_number.start = pos + 1;
                    numbers.push(current_number);
                    current_number = Number::new(row);
                }
                digit = 0;

                if character != '.' {
                    result = character.to_string();
                    symbols.push(Symbol { row, pos })
                }
            });
        if current_number.value != 0 {
            current_number.start = 0;
            numbers.push(current_number);
        }
    });

    numbers.iter().for_each(|number| {
        let mut symbols = symbols.iter().filter(|symbol| {
            let row = number.row != 0 && symbol.row == number.row - 1
                || symbol.row == number.row
                || symbol.row == number.row + 1;

            let column = number.start != 0 && symbol.pos == number.start - 1
                || symbol.pos >= number.start && symbol.pos <= number.end + 1;

            row && column
        });
        if symbols.next().is_some() {
            sum += number.value;
        }
    });

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = include_str!("example.txt");
        assert_eq!(part1(input), EXAMPLE_SOLUTION.to_string());
    }
}
