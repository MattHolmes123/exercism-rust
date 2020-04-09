pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    // Commented out original solultion
    // let mut proverbs: Vec<String> = vec![];

    // for (i, item) in list[0..list.len() - 1].iter().enumerate() {
    //     proverbs.push(format!(
    //         "For want of a {} the {} was lost.",
    //         item,
    //         list[i + 1]
    //     ));
    // }

    // proverbs.push(format!("And all for the want of a {}.", list[0]));

    // proverbs.join("\n")

    // https://doc.rust-lang.org/std/primitive.slice.html#method.windows
    list.windows(2)
        .map(|slice| format!("For want of a {} the {} was lost.", slice[0], slice[1]))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        )))
        .collect::<Vec<_>>()
        .join("\n")
}
