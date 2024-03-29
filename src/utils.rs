use std::borrow::Cow;
use anyhow::anyhow;
use clap::Parser;
use git2::{Repository, RepositoryState};
use iterator_ext::IteratorExt;
use log::{info, warn};
use rand::seq::SliceRandom;
use thiserror::Error;

pub const EMOJI_LIST: &str = "\
✊✋⬛🌍🌎🌏🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛🌜🌝🌞🌟🍇🍉🍊🍋🍌🍎🍏🍐🐃🐅🐆🐊🐋🐌🐍🐒🐔🐗🐘🐙🐛🐜🐝🐞🐟\
🐠🐡🐢🐣🐤🐥🐦🐧🐨🐪🐫🐬🐭🐮🐯🐰🐱🐳🐴🐵🐶🐷🐸🐹🐺🐻🐼🐽👀👁👆👇👈👉👊👋👌👍👏👐👻👽👿💀💨💩\
💪💫🕷🕸🖐🖖😀😁😂😃😄😆😇😈😉😊😋😌😍😎😏😐😑😒😔😕😖😗😘😙😚😛😜😝😦😧😨😪😬😮😯😱😲😳😴😵\
😶😷😸😹😺😻😼😽😾😿🙀🙂🙈🙉🙊🙌🙏🤌🤏🤐🤑🤓🤔🤗🤘🤙🤚🤛🤜🤝🤞🤟🤠🤡🤣🤤🤥🤨🤩🤪🤫🤭🤯🤲🥰🥱\
🥲🥳🥴🥶🥸🥺🦀🦁🦂🦄🦅🦆🦇🦈🦉🦊🦋🦍🦎🦏🦐🦑🦒🦓🦕🦖🦗🦘🦛🦞🦟🦣🦧🦬🦭🦾🧐🧠🪐🪰🪱🪲🪳";

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Generate a random git branch name based on remote name you given.")]
pub struct Args {
    /// remote names
    pub(crate) remote: Option<String>,

    /// local repo path
    #[arg(short = 'c', long)]
    pub(crate) repo: Option<String>,

    /// create new branch
    #[arg(short = 'b', long)]
    pub(crate) branch: bool,

    /// List all branches create by this tool and exit
    #[arg(short = 'a', long)]
    pub(crate) all: bool,

    /// verbose mode
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn check_repo(repo: &Repository) -> anyhow::Result<()> {
    // 检查仓库状态
    if repo.state() != RepositoryState::Clean {
        warn!("Warning: The repository is not clean");
    }

    // 获取当前分支名称
    let head = repo.head()?;
    if let Some(name) = head.name() {
        info!("Current branch: {}", name);
        Ok(())
    } else {
        Err(anyhow!(AppError::NotOnBranch))
    }
}

pub fn open_repo(path: &str) -> anyhow::Result<Repository> {
    let repo = Repository::discover(path)?;
    check_repo(&repo)?;
    Ok(repo)
}

pub fn create_new_branch(repo: &Repository, branch_name: Cow<str>) -> anyhow::Result<()> {
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    let new_branch = repo.branch(&*branch_name, &head_commit, false)?;
    let new_branch_ref = new_branch.into_reference();
    repo.set_head(new_branch_ref.name().unwrap())?;

    Ok(())
}

pub fn get_all_branches(repo: &Repository, remote_name: String) -> anyhow::Result<Vec<String>> {
    let remote_name_prefix = remote_name.to_string() + "/";

    let branches = repo.branches(None)?
        .try_filter_map(|(branch, _)| {
            // get name when not Error
            branch.name().map(|name| {
                // get name when not None
                name.map(|name| {
                    if branch.get().is_remote() {
                        name.strip_prefix(&remote_name_prefix)
                            .map(str::to_string)   // get own string when stripped
                    } else {
                        Some(name.to_string())
                    }
                }).flatten()    // flatten Option<Option<_>> to Option<_>
            })  // return Result<Option<String>, Error>
        });
    let res = branches.try_filter_map(|name| Ok(Some(name))).collect::<Result<_, _>>()?;
    Ok(res)
}

pub fn shuffle_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.as_mut_slice().shuffle(&mut rand::thread_rng());
    chars.into_iter().collect()
}


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not on any branch")]
    NotOnBranch,
    #[error("Multiple remotes, none specified")]
    RemoteNotSpecified,
}

#[cfg(test)]
mod tests {
    use crate::anybase::AnyBase;

    use super::*;

    #[test]
    fn test_map_ord() {
        for (i, emo) in EMOJI_LIST.char_indices() {
            info!("emoji: {}, char index: {}", emo, i);
        }

        let emojibase = AnyBase::new(EMOJI_LIST);
        macro_rules! assert_emoji_ord {
            ($emoji:expr, $ord:expr) => {
                assert_eq!(emojibase.map_ord($emoji), Some($ord));
                assert_eq!(emojibase.map_emoji($ord), $emoji);
            };
        }

        assert_emoji_ord!("✊", 1);
        assert_emoji_ord!("😎", 112);
        assert_emoji_ord!("🪳", 227);
        assert_emoji_ord!("✊✊", 228);
    }
}
