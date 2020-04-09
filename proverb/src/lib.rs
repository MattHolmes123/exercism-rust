pub fn build_proverb(list: &[&str]) -> String {
    // then all n, n + 1 pairs (until n-1)
    // then first element (for the end)

    // # https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact

    if list.is_empty() {
        return String::new();
    }

    let mut proverbs: Vec<String> = vec![];

    for (i, item) in list[0..list.len() - 1].iter().enumerate() {
        proverbs.push(format!(
            "For want of a {} the {} was lost.",
            item,
            list[i + 1]
        ));
    }

    proverbs.push(format!("And all for the want of a {}.", list[0]));

    proverbs.join("\n")
}
