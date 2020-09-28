/// Prints and spins "-=..zZ[ER]Oo..=-"
///
use std::{thread, time::Duration};

const STRING: &'static str = "..zzeroo..";

fn main() {
    let mut text: Vec<_> = STRING.split_terminator("").skip(1).collect();
    let text_len = text.len();
    let mut i = text_len;
    let middle = text_len / 2;

    loop {
        // reset counter
        if i == 0 {
            i = text_len
        };

        let text_with_clamp = text.clone();

        let mut text_with_clamp: Vec<_> = text_with_clamp
            .iter()
            .enumerate()
            .map(|(i, v)| {
                // return "." for the first and last char
                if i == 0 || i == text_len - 1 {
                    ".".to_string()
                // the 4 inner most chars are capitalized
                } else if i >= middle - 2 && i <= middle + 1 {
                    v.to_uppercase()
                } else {
                    v.to_string()
                }
            })
            .collect();

        // add header (2 char '-' and '=')
        text_with_clamp.insert(0, "-".to_string());
        text_with_clamp.insert(1, "=".to_string());

        // // add clamp
        text_with_clamp.insert(middle + 1, "[".to_string());
        text_with_clamp.insert(middle + 4, "]".to_string());

        // add footer (header + 2x clamp + footer (2 char '-' and '='))
        text_with_clamp.insert(text_len + 4, "=".to_string());
        text_with_clamp.insert(text_len + 5, "-".to_string());

        // join to a String
        let text_with_clamp = text_with_clamp.join("");

        // Clear screen
        print!("\x1B[2J");

        // print the whole thing
        println!("{:?}", &text_with_clamp);

        // rotate the text
        text.rotate_left(1);

        if i == text_len {
            thread::sleep(Duration::from_millis(800));
        } else {
            thread::sleep(Duration::from_millis(200));
        }

        // decrement counter
        i -= 1;
    }
}
