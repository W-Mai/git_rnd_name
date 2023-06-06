use anyhow::anyhow;
use clap::Parser;
use git2::{Repository, RepositoryState};
use thiserror::Error;

pub const EMOJI_LIST: &str = "\
✊✋⬛⭐️️️🌍🌎🌏🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛🌜🌝🌞🌟🍇🍉🍊🍋🍌🍎🍏🍐🐃🐅🐆🐊🐋🐌🐍🐒🐔🐗🐘🐙🐛🐜🐝🐞\
🐟🐠🐡🐢🐣🐤🐥🐦🐦🐧🐨🐪🐫🐬🐭🐮🐯🐰🐱🐳🐴🐵🐶🐷🐸🐹🐺🐻🐻🐼🐽👀👁👆👇👈👉👊👋👌👌👍👏👐👻👽\
👿💀💨💩💪💫💫🕷🕸🖐🖖😀😁😂😃😄😆😇😈😉😉😊😋😌😌😍😎😏😐😑😒😔😕😖😗😘😙😚😛😜😝😦😧😨😪😬\
😮😮😯😱😲😳😴😵😵😶😷😸😹😺😻😼😽😾😿🙀🙂🙈🙉🙊🙌🙏🤌🤏🤐🤑🤓🤔🤗🤘🤙🤚🤛🤜🤜🤝🤞🤟🤠🤡🤣🤣\
🤤🤥🤨🤩🤪🤫🤭🤯🤲🥰🥱🥲🥳🥴🥶🥸🥺🦀🦁🦂🦄🦅🦆🦇🦈🦉🦊🦋🦍🦎🦏🦐🦑🦒🦓🦕🦖🦗🦘🦛🦞🦟🦣🦧🦬🦭\
🦾🧐🧠🪐🪰🪱🪲🪳";

pub type OrdResult = Option<usize>;

pub fn map_ord(name: &str) -> OrdResult {
    let ord_base = EMOJI_LIST.chars().count();
    name.chars().try_fold(0, |res, c| {
        EMOJI_LIST.chars()
            .position(|e| e == c)
            .map(|pos| res * ord_base + pos + 1)
    })
}

pub fn map_emoji(ord: usize) -> String {
    let ord_base = EMOJI_LIST.chars().count();
    let mut ord_res = ord;
    let mut result = String::new();
    while ord_res > 0 {
        ord_res -= 1;
        let c = EMOJI_LIST.chars().nth(ord_res % ord_base).unwrap();
        result.push(c);
        ord_res /= ord_base;
    }
    result.chars().rev().collect()
}

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

pub fn check_repo(repo: &Repository) -> anyhow::Result<()> {
    // 检查仓库状态
    if repo.state() != RepositoryState::Clean {
        println!("Warning: The repository is not clean");
    }

    // 获取当前分支名称
    let head = repo.head()?;
    if let Some(name) = head.name() {
        println!("Current branch: {}", name);
        Ok(())
    } else {
        Err(anyhow!(RepoError::NotBranch))
    }
}

pub fn open_repo(path: &str) -> anyhow::Result<Repository> {
    let repo = Repository::discover(path)?;
    check_repo(&repo)?;
    Ok(repo)
}

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Not on any branch")]
    NotBranch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_ord() {
        for (i, emo) in EMOJI_LIST.char_indices() {
            println!("emoji: {}, char index: {}", emo, i);
        }

        macro_rules! assert_emoji_ord {
            ($emoji:expr, $ord:expr) => {
                assert_eq!(map_ord($emoji), Some($ord));
                assert_eq!(map_emoji($ord), $emoji);
            };
        }

        assert_emoji_ord!("✊", 1);
        assert_emoji_ord!("😎", 122);
        assert_emoji_ord!("🪳", 241);
        assert_emoji_ord!("✊✊", 242);
    }
}
