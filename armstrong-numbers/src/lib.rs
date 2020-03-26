const RADIX: u32 = 10;


pub fn is_armstrong_number(num: u32) -> bool {
    let mut total = 0;
    let num_as_str = num.to_string();
    let power = num_as_str.chars().count() as u32;

    for c in num_as_str.chars() {
        if let Some(n) = c.to_digit(RADIX) {
            total += n.pow(power);
        }
    }

    num == total
}

// Original version using a match
// for c in my_str.chars() {
//     match c.to_digit(RADIX) {
//         Some(val) => {
//             total += val.pow(power);
//         },
//         _ => println!("Nothing"),
//     }
// }