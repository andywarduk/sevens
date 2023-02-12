use atty::Stream;

use super::Card;

pub trait CardIterPrint {
    fn print(&mut self, title: &str)
    where
        Self: std::iter::Iterator<Item = Card>,
    {
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
            self.print_formatted(title, width);
        } else {
            self.print_plain(title);
        }
    }

    fn print_plain(&mut self, title: &str)
    where
        Self: std::iter::Iterator<Item = Card>,
    {
        print!("{title}");

        for c in self {
            print!(" {c}");
        }
        println!();
    }

    fn print_formatted(&mut self, title: &str, width: usize)
    where
        Self: std::iter::Iterator<Item = Card>,
    {
        let title_len = title.chars().count();

        print!("{title}");
        let mut cur_width = title_len;

        for c in self {
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
}

impl<T: Iterator<Item = Card>> CardIterPrint for T {}
