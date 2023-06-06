mod utils;
mod anybase;

use std::collections::HashSet;
use git2::BranchType;
use iterator_ext::IteratorExt;

use crate::utils::{open_repo, parse_args, shuffle_string};
use crate::anybase::{AnyBase};


fn main() -> anyhow::Result<()> {
    let args = parse_args();

    let remote_name = args.remote.as_str();
    let repo_path = args.repo.as_deref().unwrap_or(".");
    let repo = open_repo(repo_path)?;

    let branch_names = repo.branches(Some(BranchType::Remote))?
        .try_filter_map(|(branch, _)| {
            // get name when not Error
            branch.name().map(|name| {
                // get name when not None
                name.map(|name| {
                    name.strip_prefix(remote_name)
                        .map(|name| name.to_string())   // get own string when stripped
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

    println!("new-branch-name: {}", emojibase.map_emoji(new_ord));

    Ok(())
}
