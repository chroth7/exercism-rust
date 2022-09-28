pub fn verse(n: u32) -> String {
    let plural = if n > 1 { "s" } else { "" };
    let take_it = if n > 1 { "one" } else { "it" };
    let left = if n > 2 {
        format!("{} bottles", n - 1)
    } else if n > 1 {
        "1 bottle".to_string()
    } else {
        "no more bottles".to_string()
    };

    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        n => format!("{n} bottle{plural} of beer on the wall, {n} bottle{plural} of beer.\nTake {take_it} down and pass it around, {left} of beer on the wall.\n" )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    // let mut song = vec![];
    // for i in (end..=start).rev() {
    //     song.push(verse(i));
    // }
    // song.join("\n")
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
