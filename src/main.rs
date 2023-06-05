use git2::{BranchType, Repository, RepositoryState};

// const EMOJI_LIST: &str = "😋😛😝😜🤪🤨🧐🤓🥸🤩🥳😏😒😞😔😟😕🙁☹️😣😖😫😩🥺😀😃😄😁😆🤣☺️😊😇🙂😉😌😍🥰😘😗😙😚";

const REMOTE_NAME: &str = "w-mai/";

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

    // 输出所有分支名称
    println!("branches: {:?}", branch_names.collect::<Vec<String>>());
}
