use std::io::{self, Read};

fn main() {
    let mut input: Vec<u8> = Vec::new();

    io::stdin()
        .read_to_end(&mut input)
        .unwrap();

    let mut floor = 0;
    let mut hit_basement = false;
    for (idx, val) in input.iter().enumerate() {
        let brace = char::from(*val);

        if brace == '(' {
            floor += 1;
        } else if brace == ')' {
            floor -= 1;

            if floor == -1 && !hit_basement {
                hit_basement = true;
                println!("Hit basement for the first time at index {res}!!", res = idx+1);
            }
        }
    }
    println!("Send Santa to floor {floor} NOW!!!");
}
