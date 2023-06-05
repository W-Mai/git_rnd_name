use git2::{Repository, RepositoryState};

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
        },
        Err(e) => {
            println!("Error: Failed to get current branch name: {}", e);
        }
    };
}
