use anyhow::anyhow;
use clap::Parser;
use git2::{Repository, RepositoryState};
use thiserror::Error;

pub const EMOJI_LIST: &str = "\
✊✋⬛🌍🌎🌏🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛🌜🌝🌞🌟🍇🍉🍊🍋🍌🍎🍏🍐🐃🐅🐆🐊🐋🐌🐍🐒🐔🐗🐘🐙🐛🐜🐝🐞🪳\
🐟🐠🐡🐢🐣🐤🐥🐦🐦🐧🐨🐪🐫🐬🐭🐮🐯🐰🐱🐳🐴🐵🐶🐷🐸🐹🐺🐻🐻🐼🐽👀👁👆👇👈👉👊👋👌👌👍👏👐👻👽\
👿💀💨💩💪💫💫🕷🕸🖐🖖😀😁😂😃😄😆😇😈😉😉😊😋😌😌😍😎😏😐😑😒😔😕😖😗😘😙😚😛😜😝😦😧😨😪😬\
😮😮😯😱😲😳😴😵😵😶😷😸😹😺😻😼😽😾😿🙀🙂🙈🙉🙊🙌🙏🤌🤏🤐🤑🤓🤔🤗🤘🤙🤚🤛🤜🤜🤝🤞🤟🤠🤡🤣🤣\
🤤🤥🤨🤩🤪🤫🤭🤯🤲🥰🥱🥲🥳🥴🥶🥸🥺🦀🦁🦂🦄🦅🦆🦇🦈🦉🦊🦋🦍🦎🦏🦐🦑🦒🦓🦕🦖🦗🦘🦛🦞🦟🦣🦧🦬🦭\
🦾🧐🧠🪐🪰🪱🪲";

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
    use crate::anybase::AnyBase;
    use super::*;

    #[test]
    fn test_map_ord() {
        for (i, emo) in EMOJI_LIST.char_indices() {
            println!("emoji: {}, char index: {}", emo, i);
        }

        let emojibase = AnyBase::new(EMOJI_LIST);
        macro_rules! assert_emoji_ord {
            ($emoji:expr, $ord:expr) => {
                assert_eq!(emojibase.map_ord($emoji), Some($ord));
                assert_eq!(emojibase.map_emoji($ord), $emoji);
            };
        }

        assert_emoji_ord!("✊", 1);
        assert_emoji_ord!("😎", 119);
        assert_emoji_ord!("🪳", 46);
        assert_emoji_ord!("✊✊", 238);
    }
}
