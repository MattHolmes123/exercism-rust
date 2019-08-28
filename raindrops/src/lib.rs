/*
The rules of raindrops are that if a given number:

has 3 as a factor, add 'Pling' to the result.
has 5 as a factor, add 'Plang' to the result.
has 7 as a factor, add 'Plong' to the result.
does not have any of 3, 5, or 7 as a factor, the result should be the digits of the number.
*/

pub fn raindrops(n: u32) -> String {
    let is_factor_of = |f| n % f == 0;

    let raindrop_sounds: [(bool, &str); 3] = [
        (is_factor_of(3), "Pling"),
        (is_factor_of(5), "Plang"),
        (is_factor_of(7), "Plong"),
    ];

    let mut raindrop = String::new();

    for sound in raindrop_sounds.iter() {
        if sound.0 {
            raindrop.push_str(&sound.1.to_string());
        }
    }

    if raindrop.is_empty() {
        raindrop = n.to_string()
    }

    raindrop
}
