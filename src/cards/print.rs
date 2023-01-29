use atty::Stream;

use super::Card;

pub fn print_cards(iterator: impl Iterator<Item = Card>, title: &str) {
    let title_len = title.chars().count();
    let mut format = None;

    if atty::is(Stream::Stdout) {
        if let Some((w, _h)) = term_size::dimensions() {
            if w > title_len + 1 + Card::CARD_COLOURED_WIDTH {
                format = Some(w);
            }
        }
    }

    if let Some(width) = format {
        print_cards_formatted(iterator, title, width);
    } else {
        print_cards_plain(iterator, title);
    }
}

fn print_cards_plain(iterator: impl Iterator<Item = Card>, title: &str) {
    print!("{title}");

    for c in iterator {
        print!(" {c}");
    }
}

fn print_cards_formatted(iterator: impl Iterator<Item = Card>, title: &str, width: usize) {
    let title_len = title.chars().count();

    print!("{title}");
    let mut cur_width = title_len;

    for c in iterator {
        let next = c.coloured();

        if cur_width + 1 + Card::CARD_COLOURED_WIDTH > width {
            println!();
            print!("{:>title_len$}", "");
            cur_width = title_len;
        }

        print!(" {next}");
        cur_width += 1 + Card::CARD_COLOURED_WIDTH;
    }

    println!();
}
