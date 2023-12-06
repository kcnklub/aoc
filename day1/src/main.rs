use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("couldn't read input file");

    let mut answer = 0;
    for line in file.lines() {
        let number = number(line);
        println!("{}", number);

        answer += number;
    }

    println!("answer: {}", answer)
}

fn number(line: &str) -> u32 {
    let first_letters = ['o', 't', 'f', 's', 'e', 'n'];
    let mut ff = false;
    let mut f = '0';
    let mut l = '0';
    let chars = line.chars();
    for (i, char) in chars.clone().enumerate() {
        if char.is_numeric() {
            if !ff {
                f = char;
                ff = true;
            }
            l = char;
        } else {
            // this charater is the first letter of a number
            let mut new = None;
            match char {
                'o' => {
                    let temp = chars.as_str().get(i..i + 3);
                    if let Some(value) = temp {
                        if value == "one" {
                            new = Some('1');
                        }
                    }
                }
                't' => {
                    let temp = chars.as_str().get(i..i + 2);
                    if let Some(value) = temp {
                        if value == "th" {
                            if let Some(value) = chars.as_str().get(i..i + 5) {
                                if value == "three" {
                                    new = Some('3');
                                }
                            }
                        } else {
                            if let Some(value) = chars.as_str().get(i..i + 3) {
                                if value == "two" {
                                    new = Some('2');
                                }
                            }
                        }
                    }
                }
                'f' => {
                    let temp = chars.as_str().get(i..i + 4);
                    if let Some(value) = temp {
                        if value == "four" {
                            new = Some('4');
                        } else if value == "five" {
                            new = Some('5');
                        }
                    }
                }
                's' => {
                    let temp = chars.as_str().get(i..i + 2);
                    if let Some(value) = temp {
                        if value == "si" {
                            if let Some(value) = chars.as_str().get(i..i + 3) {
                                if value == "six" {
                                    new = Some('6');
                                }
                            }
                        } else {
                            if let Some(value) = chars.as_str().get(i..i + 5) {
                                if value == "seven" {
                                    new = Some('7');
                                }
                            }
                        }
                    }
                }
                'e' => {
                    let temp = chars.as_str().get(i..i + 5);
                    if let Some(value) = temp {
                        if value == "eight" {
                            new = Some('8');
                        }
                    }
                }

                'n' => {
                    let temp = chars.as_str().get(i..i + 4);
                    if let Some(value) = temp {
                        if value == "nine" {
                            new = Some('9');
                        }
                    }
                }
                _ => (),
            }

            if let Some(value) = new {
                if !ff {
                    f = value;
                    ff = true;
                }
                l = value;
            }
        }
    }

    let string = String::from_utf8(vec![f as u8, l as u8]).unwrap();

    string.parse::<u32>().unwrap()
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let input = "msixonexch1twokjbdlhchqk1";
        let number = super::number(input);

        assert_eq!(number, 61);
    }
}
