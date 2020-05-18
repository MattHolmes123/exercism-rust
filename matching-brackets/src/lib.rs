pub fn brackets_are_balanced(string: &str) -> bool {
    let all_brackets = vec!['[', ']', '{', '}', '(', ')'];
    let mut brackets = vec![];

    for x in string.chars(){
        match x {
            x if all_brackets.contains(&x) => brackets.push(x),
            _ => {}
        }
    }

    // take first and last item and compare
    let len = brackets.len();

    if len % 2 != 0 {
        return false
    }

    // FIXME: Sort this out.
    // let nested = [
    //     "[]", "{}",
    // ];
    // filter nested
    // brackets = brackets.chunks(2)
    //     .filter(
    //         |pair| !nested.contains(
    //             format!("{}{}", pair)
    //         )
    //     )
    //     .collect();

    let (opening, closing) = brackets.split_at_mut(len/2);

    closing.reverse();

    for (o, c) in opening.iter().zip(closing.iter()) {
        let mut pair_valid = false;

        if o == &'[' {
            pair_valid = c == &']';
        }

        if o == &'{' {
            pair_valid = c == &'}'
        }

        if o == &'(' {
            pair_valid = c == &')'
        }

        if !pair_valid {
            return false
        }
    }

    return true
}
