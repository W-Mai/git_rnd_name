pub const EMOJI_LIST: &str = "\
✊✋⬛⭐️️️🌍🌎🌏🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛🌜🌝🌞🌟🍇🍉🍊🍋🍌🍎🍏🍐🐃🐅🐆🐊🐋🐌🐍🐒🐔🐗🐘🐙🐛🐜🐝🐞\
🐟🐠🐡🐢🐣🐤🐥🐦🐦🐧🐨🐪🐫🐬🐭🐮🐯🐰🐱🐳🐴🐵🐶🐷🐸🐹🐺🐻🐻🐼🐽👀👁👆👇👈👉👊👋👌👌👍👏👐👻👽\
👿💀💨💩💪💫💫🕷🕸🖐🖖😀😁😂😃😄😆😇😈😉😉😊😋😌😌😍😎😏😐😑😒😔😕😖😗😘😙😚😛😜😝😦😧😨😪😬\
😮😮😯😱😲😳😴😵😵😶😷😸😹😺😻😼😽😾😿🙀🙂🙈🙉🙊🙌🙏🤌🤏🤐🤑🤓🤔🤗🤘🤙🤚🤛🤜🤜🤝🤞🤟🤠🤡🤣🤣\
🤤🤥🤨🤩🤪🤫🤭🤯🤲🥰🥱🥲🥳🥴🥶🥸🥺🦀🦁🦂🦄🦅🦆🦇🦈🦉🦊🦋🦍🦎🦏🦐🦑🦒🦓🦕🦖🦗🦘🦛🦞🦟🦣🦧🦬🦭\
🦾🧐🧠🪐🪰🪱🪲🪳";

#[derive(Debug, PartialEq)]
pub enum OrdResult {
    Ord(i32),
    Invalid,
}


pub fn map_ord(name: &str) -> OrdResult {
    fn get_order(c: char) -> OrdResult {
        let pos = EMOJI_LIST.char_indices().position(|(_, e)| {
            e == c
        });
        match pos {
            Some(i) => OrdResult::Ord((i + 1) as i32),
            None => OrdResult::Invalid,
        }
    }

    let ord_base = EMOJI_LIST.char_indices().count() as i32;
    let mut ord_res = 0;
    for c in name.char_indices() {
        let ord = match get_order(c.1) {
            OrdResult::Ord(i) => {
                i
            }
            OrdResult::Invalid => { return OrdResult::Invalid; }
        };

        ord_res = ord_res * ord_base + ord;
    }
    OrdResult::Ord(ord_res)
}

pub fn map_emoji(ord: i32) -> String {
    let ord_base = EMOJI_LIST.char_indices().count() as i32;
    let mut ord_res = ord;
    let mut result = String::new();
    while ord_res > 0 {
        ord_res -= 1;
        let c = EMOJI_LIST.char_indices().nth((ord_res % ord_base) as usize).unwrap().1;
        result.push(c as char);
        ord_res /= ord_base;
    }
    result.chars().rev().collect()
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Generate a random git branch name based on remote name you given.")]
pub struct Args {
    /// remote names
    pub(crate) remote: String,

    /// local repo path
    #[arg(short = 'c', long)]
    pub(crate) repo: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_ord() {
        for emo in EMOJI_LIST.char_indices() {
            println!("emoji: {}, char index: {}", emo.1, emo.0);
        }

        assert_eq!(map_ord("😋"), OrdResult::Ord(1));
        assert_eq!(map_emoji(1), "😋");
        assert_eq!(map_ord("😛"), OrdResult::Ord(2));
        assert_eq!(map_emoji(2), "😛");
        assert_eq!(map_ord("😎"), OrdResult::Ord(43));
        assert_eq!(map_emoji(43), "😎");
    }
}
