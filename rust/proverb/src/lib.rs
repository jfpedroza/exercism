pub fn build_proverb(list: &[&str]) -> String {
    match list {
        [] => String::new(),
        [el] => format!("And all for the want of a {}.", el),
        _ => {
            let before = (0..list.len() - 1)
                .map(|i| format!("For want of a {} the {} was lost.", list[i], list[i + 1]))
                .collect::<Vec<_>>()
                .join("\n");
            let after = format!("And all for the want of a {}.", list.first().unwrap());
            format!("{}\n{}", before, after)
        }
    }
}
