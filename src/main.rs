use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("test.tur").unwrap();

    let mut strip: u64 = 0;
    let mut pointer_location: u8 = 63;
    let mut selected_bit: u8 = 0;

    let lines: Vec<&str> = file.lines().collect();
    let mut line_index: usize = 0;

    'lineloop: while line_index < lines.len() {
        let l = lines[line_index];
        let chars: Vec<char> = l.chars().collect();

        let mut i = 0;
        'charloop: while i < chars.len() {
            let c = chars[i];
            match c {
                'p' => {
                    // print
                    println!("{:064b}", strip); // print the strip
                    println!("{}^", " ".repeat((63 - pointer_location) as usize));
                    // print the cursor
                }
                // right and left move in the "wrong" direction because u64 is stored from right to left
                '>' => {
                    // move pointer right
                    if pointer_location as i64 - 1 < 0 {
                        println!("Pointer went over 64!");
                        return;
                    }
                    pointer_location -= 1;
                }
                '<' => {
                    // move pointer left
                    if pointer_location as i64 + 1 > 64 {
                        println!("Pointer went under 0!");
                        return;
                    }
                    pointer_location += 1;
                }
                't' => strip ^= 1 << pointer_location, // toggle bit
                '+' => strip |= 1 << pointer_location, // toggle bit on
                '-' => strip &= !(1 << pointer_location), // toggle bit off
                'l' => {
                    // load bit to selected bit
                    let mask = 1 << pointer_location;

                    selected_bit = if mask & strip == 1 << pointer_location {
                        1
                    } else {
                        0
                    };
                }
                '?' => {
                    // syntax:
                    // ? 1: 2
                    //   ^ line to jump to if current bit is 1
                    //     ^ line to jump to if current bit is 0

                    let re = Regex::new(r"(?<trueline>\d+) *: *(?<falseline>\d+)").unwrap();

                    let Some(caps) = re.captures(l) else {
                        println!("invalid if statement on line {}", line_index + 1);
                        return;
                    };

                    if selected_bit == 1 {
                        line_index = caps["trueline"].parse::<usize>().unwrap() - 1;
                    } else {
                        line_index = caps["falseline"].parse::<usize>().unwrap() - 1;
                    }
                    continue 'lineloop;
                }
                ';' => break 'charloop, // comment, can ignore rest of the line
                ' ' => (),              // ignore whitespace
                _ => {
                    println!("Unknown char: {:?} on line: {}", c, line_index);
                    return;
                }
            }
            i += 1;
        }
        line_index += 1;
    }
}
