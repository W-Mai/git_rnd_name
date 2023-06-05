use git2::{BranchType, Repository, RepositoryState};

const EMOJI_LIST: &str = "😋😛😝😜🤪🤨🧐🤓🥸🤩🥳😏😒😞😔😟😕🙁😣😖😫😩🥺😀😃😄😁😆🤣😊😇🙂😉😌😍🥰😘😗😙😚🫢👌😎";

const REMOTE_NAME: &str = "w-mai/";

#[derive(Debug, PartialEq)]
enum OrdResult {
    Ord(i32),
    Invalid,
}

fn map_ord(c: &str) -> OrdResult {
    fn get_order(c: &str) -> OrdResult {
        let pos = EMOJI_LIST.char_indices().position(|(_, e)| {
            e == c.chars().nth(0).unwrap()
        });
        match pos {
            Some(i) => OrdResult::Ord(i as i32),
            None => OrdResult::Invalid,
        }
    }
    get_order(c)
}

fn main() {
    // 打开当前目录下的 Git 仓库

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
    });

    let branch_indexes = branch_names.map(|branch_name| {
        let ord = map_ord(branch_name.as_str());
        match ord {
            OrdResult::Ord(i) => {
                i
            }
            OrdResult::Invalid => { -1 }
        }
    });
    // 输出所有分支名称
    println!("branches: {:?}", branch_indexes.collect::<Vec<i32>>());
}

// write a test for map_ord function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_ord() {
        for emo in EMOJI_LIST.char_indices() {
            println!("emoji: {}, index: {}", emo.1, emo.0);
        }

        assert_eq!(map_ord("😋"), OrdResult::Ord(0));
        assert_eq!(map_ord("😛"), OrdResult::Ord(1));
        assert_eq!(map_ord("😝"), OrdResult::Ord(2));
        assert_eq!(map_ord("😜"), OrdResult::Ord(3));
        assert_eq!(map_ord("🤪"), OrdResult::Ord(4));
        assert_eq!(map_ord("🤨"), OrdResult::Ord(5));
        assert_eq!(map_ord("🧐"), OrdResult::Ord(6));
        assert_eq!(map_ord("🤓"), OrdResult::Ord(7));
        assert_eq!(map_ord("🥸"), OrdResult::Ord(8));
        assert_eq!(map_ord("🤩"), OrdResult::Ord(9));
        assert_eq!(map_ord("🥳"), OrdResult::Ord(10));
        assert_eq!(map_ord("😏"), OrdResult::Ord(11));
        assert_eq!(map_ord("😒"), OrdResult::Ord(12));
        assert_eq!(map_ord("😞"), OrdResult::Ord(13));
        assert_eq!(map_ord("😔"), OrdResult::Ord(14));
        assert_eq!(map_ord("😟"), OrdResult::Ord(15));
        assert_eq!(map_ord("😕"), OrdResult::Ord(16));
        assert_eq!(map_ord("🙁"), OrdResult::Ord(17));
        assert_eq!(map_ord("😣"), OrdResult::Ord(18));
        assert_eq!(map_ord("😖"), OrdResult::Ord(19));
        assert_eq!(map_ord("😫"), OrdResult::Ord(20));
        assert_eq!(map_ord("😩"), OrdResult::Ord(21));
        assert_eq!(map_ord("🥺"), OrdResult::Ord(22));
        assert_eq!(map_ord("😀"), OrdResult::Ord(23));
        assert_eq!(map_ord("😃"), OrdResult::Ord(24));
        assert_eq!(map_ord("😄"), OrdResult::Ord(25));
        assert_eq!(map_ord("😁"), OrdResult::Ord(26));
        assert_eq!(map_ord("😆"), OrdResult::Ord(27));
        assert_eq!(map_ord("🤣"), OrdResult::Ord(28));
        assert_eq!(map_ord("😊"), OrdResult::Ord(29));
        assert_eq!(map_ord("😇"), OrdResult::Ord(30));
        assert_eq!(map_ord("🙂"), OrdResult::Ord(31));
        assert_eq!(map_ord("😉"), OrdResult::Ord(32));
        assert_eq!(map_ord("😌"), OrdResult::Ord(33));
        assert_eq!(map_ord("😍"), OrdResult::Ord(34));
        assert_eq!(map_ord("🥰"), OrdResult::Ord(35));
        assert_eq!(map_ord("😘"), OrdResult::Ord(36));
        assert_eq!(map_ord("😗"), OrdResult::Ord(37));
        assert_eq!(map_ord("😙"), OrdResult::Ord(38));
        assert_eq!(map_ord("😚"), OrdResult::Ord(39));
        assert_eq!(map_ord("哈哈"), OrdResult::Invalid);
    }
}
