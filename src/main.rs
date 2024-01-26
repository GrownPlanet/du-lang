use std::fs;

fn main() {
    let file = fs::read_to_string("test.tur").unwrap();

    let mut strip: u32 = 0;
    let mut pointer_location: u8 = 0;

    for l in file.lines() {
        'charloop: for c in l.chars() {
            match c {
                'p' => {
                    // print
                    println!("{:032b}", strip); // print the strip
                    println!("{}^", " ".repeat(pointer_location as usize)); // print the cursor
                }
                '>' => pointer_location += 1, // move pointer right
                '<' => pointer_location -= 1, // move pointer left
                't' => strip ^= 1 << 31 - pointer_location, // toggle bit
                '+' => strip |= 1 << 31 - pointer_location, // toggle bit on
                '-' => strip &= !(1 << 31 - pointer_location), // toggle bit off
                'c' => {
                    // compare bit
                    todo!()
                }
                ';' => break 'charloop, // comment, can ignore rest of the line
                ' ' => (),              // ignore whitespace
                _ => println!("Unknown char: {:?}", c),
            }
        }
    }
}
