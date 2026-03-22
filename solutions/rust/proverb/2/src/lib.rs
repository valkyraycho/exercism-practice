pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .chain(
            list.first()
                .map(|&first| format!("And all for the want of a {}.", first)),
        )
        .collect::<Vec<_>>()
        .join("\n")
}
