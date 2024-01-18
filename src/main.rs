use std::collections::HashSet;

use anyhow::anyhow;
use env_logger::{Builder, Env};
use iterator_ext::IteratorExt;

use crate::anybase::AnyBase;
use crate::utils::{AppError, create_new_branch, open_repo, parse_args, shuffle_string};

mod utils;
mod anybase;

fn main() -> anyhow::Result<()> {
    let args = parse_args();

    Builder::from_env(Env::default().default_filter_or(match args.verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    })).init();

    let repo_path = args.repo.as_deref().unwrap_or(".");
    let repo = open_repo(repo_path)?;

    // Obtain remote name from args or use default
    let remote_name = if let Some(name) = args.remote { name } else {
        let remotes = repo.remotes()?;
        if remotes.len() != 1 {
            return Err(anyhow!(AppError::RemoteNotSpecified));
        }
        remotes.get(0).unwrap().to_string()
    };
    let remote_name_prefix = remote_name.to_string() + "/";

    let branch_names = repo.branches(None)?
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

    let emojibase = AnyBase::new(shuffle_string(utils::EMOJI_LIST).as_str());

    let branch_ords: HashSet<_> = branch_names
        .try_filter_map(|name| Ok(emojibase.map_ord(&name)))
        .collect::<Result<_, _>>()?;    // throw the error if exists

    let mut new_ord = 1;
    while branch_ords.contains(&new_ord) {
        new_ord += 1;
    }

    let new_branch_name = emojibase.map_emoji(new_ord);


    if args.branch {
        create_new_branch(&repo, new_branch_name.clone())?;   // create new branch
        println!("{} was created.", new_branch_name);
    } else {
        println!("{}", new_branch_name);
    }

    Ok(())
}
