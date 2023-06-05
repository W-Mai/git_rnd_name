use std::collections::{HashMap, HashSet};
use git2::{BranchType, Repository, RepositoryState};

const EMOJI_LIST: &str = "\
✊✋⬛⭐️️️🌍🌎🌏🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛🌜🌝🌞🌟🍇🍉🍊🍋🍌🍎🍏🍐🐃🐅🐆🐊🐋🐌🐍🐒🐔🐗🐘🐙🐛🐜🐝🐞\
🐟🐠🐡🐢🐣🐤🐥🐦🐦🐧🐨🐪🐫🐬🐭🐮🐯🐰🐱🐳🐴🐵🐶🐷🐸🐹🐺🐻🐻🐼🐽👀👁👆👇👈👉👊👋👌👌👍👏👐👻👽\
👿💀💨💩💪💫💫🕷🕸🖐🖖😀😁😂😃😄😆😇😈😉😉😊😋😌😌😍😎😏😐😑😒😔😕😖😗😘😙😚😛😜😝😦😧😨😪😬\
😮😮😯😱😲😳😴😵😵😶😷😸😹😺😻😼😽😾😿🙀🙂🙈🙉🙊🙌🙏🤌🤏🤐🤑🤓🤔🤗🤘🤙🤚🤛🤜🤜🤝🤞🤟🤠🤡🤣🤣\
🤤🤥🤨🤩🤪🤫🤭🤯🤲🥰🥱🥲🥳🥴🥶🥸🥺🦀🦁🦂🦄🦅🦆🦇🦈🦉🦊🦋🦍🦎🦏🦐🦑🦒🦓🦕🦖🦗🦘🦛🦞🦟🦣🦧🦬🦭\
🦾🧐🧠🪐🪰🪱🪲🪳";

const REMOTE_NAME: &str = "w-mai/";

#[derive(Debug, PartialEq)]
enum OrdResult {
    Ord(i32),
    Invalid,
}


fn map_ord(name: &str) -> OrdResult {
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

fn map_emoji(ord: i32) -> String {
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

fn main() {
    // 打开当前目录下的 Git 仓库
    let repo = Repository::discover(".").unwrap();

    // 检查仓库状态
    match repo.state() {
        RepositoryState::Clean => {
            println!("Warning: The repository is not clean");
        }
        _ => {}
    };

    // 获取当前分支名称
    match repo.head() {
        Ok(reference) => {
            if let Some(name) = reference.name() {
                println!("Current branch: {}", name);
            } else {
                println!("Not on any branch");
            }
        }
        Err(e) => {
            println!("Error: Failed to get current branch name: {}", e);
        }
    };

    let branches = repo.branches(Some(BranchType::Remote)).unwrap().filter(|branch| {
        match branch {
            Ok((branch, _)) => {
                let name = branch.name().unwrap().unwrap();
                name.starts_with(REMOTE_NAME)
            }
            Err(e) => {
                println!("Error: Failed to get branch name: {}", e);
                false
            }
        }
    });

    let branch_names = branches.map(|branch| {
        let (branch, _) = branch.unwrap();
        branch.name().unwrap().unwrap().strip_prefix(REMOTE_NAME).unwrap().to_string()
    }).collect::<Vec<String>>();

    let mut branch_name_set = HashSet::new();
    for branch_name in branch_names.clone() {
        branch_name_set.insert(branch_name);
    }

    let branch_name_ord_map = branch_names.clone().into_iter().map(|branch_name| {
        let ord = map_ord(branch_name.as_str());
        match ord {
            OrdResult::Ord(i) => {
                (branch_name, i)
            }
            OrdResult::Invalid => {
                (branch_name, -1)
            }
        }
    }).collect::<HashMap<String, i32>>();

    let mut branch_ords = branch_name_ord_map.values().clone().filter(|i| {
        **i != -1
    }).map(|i| {
        *i
    }).collect::<Vec<i32>>();

    branch_ords.sort();

    let branch_ords = branch_ords.into_iter().collect::<HashSet<i32>>();

    let mut new_ord = 1;

    while branch_ords.contains(&new_ord) || branch_name_set.contains(&map_emoji(new_ord)) {
        new_ord += 1;
    }

    println!("new-branch-name: {}", map_emoji(new_ord));
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
