const TEXT: &str = "%s of beer on the wall, %s of beer.\n%s, %s of beer on the wall.\n";
const ONE_BOTTLE: &str = "1 bottle";

pub fn verse(n: u32) -> String {
    let mut pat1 = format!("{} bottles", n);
    let mut pat2 = pat1.clone();
    let mut pat3 = String::from("Take one down and pass it around");
    let mut pat4 = ONE_BOTTLE.to_string();

    match n {
        n if n == 1 => pat1 = ONE_BOTTLE.to_string(),
        n if n == 0 => pat1 = String::from("No more bottles"),
        _ => (),
    }

    match n {
        n if n == 1 => pat2 = ONE_BOTTLE.to_string(),
        n if n == 0 => pat2 = pat1.to_lowercase(),
        _ => (),
    }

    match n {
        n if n == 1 => {
            pat3 = String::from("Take it down and pass it around");
        }
        n if n == 0 => {
            pat3 = String::from("Go to the store and buy some more");
        }
        _ => (),
    }

    match n {
        n if n > 2 => {
            pat4 = format!("{} bottles", n - 1);
        }
        n if n == 1 => {
            pat4 = String::from("no more bottles");
        }
        n if n == 0 => {
            pat4 = String::from("99 bottles");
        }
        _ => (),
    }

    let text = TEXT.replacen("%s", &pat1, 1);
    let text = text.replacen("%s", &pat2, 1);
    let text = text.replacen("%s", &pat3, 1);
    text.replacen("%s", &pat4, 1)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut i: u32 = start;

    let mut song: String = String::from("");
    while i >= end {
        song.push_str(&verse(i));

        if i > end {
            i -= 1;
            song.push('\n');
        } else {
            break;
        }
    }

    song
}
