mod utils;

use std::collections::{HashMap, HashSet};
use git2::{BranchType, Repository, RepositoryState};
use crate::utils::{map_emoji, map_ord, OrdResult, parse_args};

fn main() {
    let args = parse_args();

    let remote_name = args.remote.clone();
    let repo_path = args.repo.clone().unwrap_or(".".to_string());

    // 打开当前目录下的 Git 仓库
    let repo = Repository::discover(repo_path).unwrap();

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
                name.starts_with(remote_name.as_str())
            }
            Err(e) => {
                println!("Error: Failed to get branch name: {}", e);
                false
            }
        }
    });

    let branch_names = branches.map(|branch| {
        let (branch, _) = branch.unwrap();
        branch.name().unwrap().unwrap().strip_prefix(remote_name.as_str()).unwrap().to_string()
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
