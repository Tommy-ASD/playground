/*
 * libgit2 "pull" example - shows how to pull remote data into a local branch.
 *
 * Written by the libgit2 contributors
 *
 * To the extent possible under law, the author(s) have dedicated all copyright
 * and related and neighboring rights to this software to the public domain
 * worldwide. This software is distributed without any warranty.
 *
 * You should have received a copy of the CC0 Public Domain Dedication along
 * with this software. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 */

use git2::Repository;
use std::collections::HashSet;
use std::io::{self, Write};
use std::path::PathBuf;
use std::str;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
    #[arg(short, long)]
    source: Option<String>,
    #[arg(short, long)]
    branch: Option<String>,
    #[arg(short, long)]
    recursive: bool,
}

fn do_fetch<'a>(
    repo: &'a git2::Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();

    // Print out our transfer progress.
    cb.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        io::stdout().flush().unwrap();
        true
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);
    // Always fetch all tags.
    // Perform a download and also update tips
    fo.download_tags(git2::AutotagOption::All);
    println!("Fetching {} for repo", remote.name().unwrap());
    remote.fetch(refs, Some(&mut fo), None).unwrap();

    // If there are local objects (we got a thin pack), then tell the user
    // how many objects we saved from having to cross the network.
    let stats = remote.stats();
    if stats.local_objects() > 0 {
        println!(
            "\rReceived {}/{} objects in {} bytes (used {} local \
             objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        println!(
            "\rReceived {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    Ok(repo.reference_to_annotated_commit(&fetch_head).unwrap())
}

fn fast_forward(
    repo: &Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    println!("{}", msg);
    lb.set_target(rc.id(), &msg).unwrap();
    repo.set_head(&name).unwrap();
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an example so maybe not.
            .force(),
    ))
    .unwrap();
    Ok(())
}

fn normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id()).unwrap().tree().unwrap();
    let remote_tree = repo.find_commit(remote.id()).unwrap().tree().unwrap();
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id()).unwrap())
        .unwrap()
        .tree()
        .unwrap();
    let mut idx = repo
        .merge_trees(&ancestor, &local_tree, &remote_tree, None)
        .unwrap();

    if idx.has_conflicts() {
        println!("Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None).unwrap();
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo).unwrap()).unwrap();
    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature().unwrap();
    let local_commit = repo.find_commit(local.id()).unwrap();
    let remote_commit = repo.find_commit(remote.id()).unwrap();
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo
        .commit(
            Some("HEAD"),
            &sig,
            &sig,
            &msg,
            &result_tree,
            &[&local_commit, &remote_commit],
        )
        .unwrap();
    // Set working tree to match head.
    repo.checkout_head(None).unwrap();
    Ok(())
}

fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: git2::AnnotatedCommit<'a>,
) {
    // 1. do a merge analysis
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();

    // 2. Do the appropriate merge
    if analysis.0.is_fast_forward() {
        println!("Doing a fast forward");
        // do a fast forward
        let refname = format!("refs/heads/{}", remote_branch);
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                fast_forward(repo, &mut r, &fetch_commit).unwrap();
            }
            Err(_) => {
                // The branch doesn't exist so just set the reference to the
                // commit directly. Usually this is because you are pulling
                // into an empty repository.
                repo.reference(
                    &refname,
                    fetch_commit.id(),
                    true,
                    &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                )
                .unwrap();
                repo.set_head(&refname).unwrap();
                repo.checkout_head(Some(
                    git2::build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))
                .unwrap();
            }
        };
    } else if analysis.0.is_normal() {
        // do a normal merge
        let head_commit = repo
            .reference_to_annotated_commit(&repo.head().unwrap())
            .unwrap();
        normal_merge(&repo, &head_commit, &fetch_commit).unwrap();
    } else {
        println!("Nothing to do...");
    }
}

fn run(repo_path: &PathBuf, remote_name: &str, remote_branch: &str) -> Result<(), git2::Error> {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Path {repo_path:?} is not a repo");
            return Err(e);
        }
    };
    let mut remote = repo.find_remote(remote_name)?;
    let fetch_commit = do_fetch(&repo, &[remote_branch], &mut remote)?;
    Ok(do_merge(&repo, &remote_branch, fetch_commit))
}

async fn walk_dir(dir: PathBuf) -> Vec<PathBuf> {
    let ignore: Vec<PathBuf> = vec!["target".into(), ".git".into(), ".vscode".into()];

    let mut dirs_iter = vec![dir.clone()];
    let mut dirs = vec![dir.clone()];

    println!("Initialized dirs as {dirs_iter:?}");

    while !dirs_iter.is_empty() {
        let dir_path = dirs_iter.remove(0);
        println!("Checking dir {dir_path:?}");

        let mut dir_iter = tokio::fs::read_dir(dir_path).await.unwrap();

        while let Some(entry) = dir_iter.next_entry().await.unwrap() {
            let entry_path_buf = entry.path();

            if entry_path_buf.is_dir() {
                let mut skip = false;
                for i in &ignore {
                    if entry_path_buf.ends_with(&i) {
                        skip = true;
                        break;
                    }
                }
                if skip {
                    continue;
                }

                println!("Found dir {entry_path_buf:?}");

                dirs_iter.push(entry_path_buf.clone());
                dirs.push(entry_path_buf.clone());
            } else {
                println!("Found entry {entry_path_buf:?}");
            }
        }
    }

    dedupe_vec(dirs)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let remote_name = args.source.as_ref().map(|s| &s[..]).unwrap_or("origin");
    let remote_branch = args.branch.as_ref().map(|s| &s[..]).unwrap_or("master");
    let path = args.path.as_ref().map(|s| &s[..]).unwrap_or(".").into();
    if args.recursive {
        let mut tasks = vec![];
        let dirs = walk_dir(path).await;
        println!("Found dirs {dirs:?}");
        for dir in dirs {
            let rname = remote_name.to_string();
            let rbranch = remote_branch.to_string();
            tasks.push(tokio::task::spawn_blocking(move || {
                run(&dir, &rname, &rbranch)
            }));
        }
        futures::future::join_all(tasks).await;
    } else {
        match run(&path, remote_name, remote_branch) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

fn dedupe_vec<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = vec.into_iter().collect();
    let deduped_vec: Vec<_> = set.into_iter().collect();
    deduped_vec
}
