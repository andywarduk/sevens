use std::io::{stdout, IsTerminal};

use terminal_size::Width;

use super::Card;

pub trait CardIterPrint: Iterator<Item = Card> {
    fn print(&mut self, title: &str) {
        let title_len = title.chars().count();
        let mut format = None;

        if stdout().is_terminal() {
            if let Some((Width(w), _)) = terminal_size::terminal_size() {
                if w as usize > title_len + 1 + Card::CARD_COLOURED_WIDTH {
                    format = Some(w);
                }
            }
        }

        if let Some(width) = format {
            self.print_formatted(title, width as usize);
        } else {
            self.print_plain(title);
        }
    }

    fn print_plain(&mut self, title: &str) {
        print!("{title}");

        for c in self {
            print!(" {c}");
        }
        println!();
    }

    fn print_formatted(&mut self, title: &str, width: usize) {
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
