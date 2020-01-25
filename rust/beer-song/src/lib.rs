pub fn verse(n: u32) -> String {
    format!("{}\n{}\n", first_sentence(n), second_sentence(n))
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses: Vec<_> = (end..=start).map(verse).collect();
    verses.reverse();

    verses.join("\n")
}

fn first_sentence(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer."),
        _ => format!(
            "{} of beer on the wall, {} of beer.",
            format_bottle(n),
            format_bottle(n)
        ),
    }
}

fn second_sentence(n: u32) -> String {
    match n {
        0 => String::from("Go to the store and buy some more, 99 bottles of beer on the wall."),
        1 => String::from("Take it down and pass it around, no more bottles of beer on the wall."),
        _ => format!(
            "Take one down and pass it around, {} of beer on the wall.",
            format_bottle(n - 1)
        ),
    }
}

fn format_bottle(n: u32) -> String {
    match n {
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n),
    }
}
