mod utils;
mod anybase;

use std::collections::HashSet;
use anyhow::anyhow;
use iterator_ext::IteratorExt;
use env_logger::{Builder, Env};

use crate::utils::{create_new_branch, open_repo, parse_args, shuffle_string};
use crate::anybase::{AnyBase};

fn main() -> anyhow::Result<()> {
    let args = parse_args();

    Builder::from_env(Env::default().default_filter_or(match args.verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        _ => "trace",
    })).init();

    let remote_name = args.remote + "/";
    let remote_name = remote_name.as_str();
    let repo_path = args.repo.as_deref().unwrap_or(".");
    let repo = open_repo(repo_path)?;

    let branch_names = repo.branches(None)?
        .try_filter_map(|(branch, _)| {
            // get name when not Error
            branch.name().map(|name| {
                // get name when not None
                name.map(|name| {
                    if branch.get().is_remote() {
                        name.strip_prefix(remote_name)
                            .map(|name| name.to_string())   // get own string when stripped
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

    if branch_ords.len() == 0 {
        return Err(anyhow!("No remote branch found for {}", remote_name));
    }

    let mut new_ord = 1;
    while branch_ords.contains(&new_ord) {
        new_ord += 1;
    }

    let new_branch_name = emojibase.map_emoji(new_ord);


    if args.branch {
        create_new_branch(&repo, &new_branch_name)?;   // create new branch
        println!("{} was created.", new_branch_name);
    } else {
        println!("{}", new_branch_name);
    }

    Ok(())
}
