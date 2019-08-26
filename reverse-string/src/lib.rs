extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let result: String = input.graphemes(true).rev().collect();

    result
}

//pub fn v1_reverse(input: &str) -> String {
//    Allocate the correct memory amount.
//    let capacity = input.len();
//    let mut result = String::with_capacity(capacity);
//
//    for c in input.chars().rev() {
//        result.push(c);
//    }
//
//    result
//}

//pub fn v2_reverse(input: &str) -> String {
//    let result: String = input.chars().rev().collect();
//    result
//}
