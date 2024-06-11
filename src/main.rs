use std::collections::HashSet;

use anyhow::anyhow;
use env_logger::{Builder, Env};

use crate::anybase::AnyBase;
use crate::utils::{
    create_new_branch, get_all_branches, open_repo, parse_args, shuffle_string, AppError,
};

mod anybase;
mod utils;

fn main() -> anyhow::Result<()> {
    let args = parse_args();

    Builder::from_env(Env::default().default_filter_or(match args.verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    }))
    .init();

    let repo_path = args.repo.as_deref().unwrap_or(".");
    let repo = open_repo(repo_path)?;

    // Obtain remote name from args or use default
    let remote_name = if let Some(name) = args.remote {
        name
    } else {
        let remotes = repo.remotes()?;
        if remotes.len() != 1 {
            return Err(anyhow!(AppError::RemoteNotSpecified));
        }
        remotes.get(0).unwrap().to_string()
    };

    let branch_names = get_all_branches(&repo, remote_name.clone())?;

    let emojibase = AnyBase::new(shuffle_string(utils::EMOJI_LIST).as_str());

    let branch_ords: HashSet<_> = branch_names
        .iter()
        .filter_map(|name| emojibase.map_ord(name))
        .collect::<_>();

    let mut new_ord = 1;
    while branch_ords.contains(&new_ord) {
        new_ord += 1;
    }

    let new_branch_name = emojibase.map_emoji(new_ord);
    

    if args.all {
        branch_ords.iter().for_each(|ord| {
            let emoji = emojibase.map_emoji(*ord);
            println!("{}", emoji);
        });
    } else {
        if args.branch {
            create_new_branch(&repo, new_branch_name.clone())?; // create new branch
            println!("{} was created.", new_branch_name);
        } else {
            println!("{}", new_branch_name);
        }
    }
    Ok(())
}
