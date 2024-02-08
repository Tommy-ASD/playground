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

use async_recursion::async_recursion;
use core::panic;
use git2::Repository;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    io::{self, Write},
    path::PathBuf,
    str::{self, FromStr},
    sync::Arc,
};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration, Instant},
};
use url::Url;

use clap::Parser;

static GLOBAL_TIMEOUT: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));
static IS_PAUSED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    user: String,
    #[arg(short, long, default_value = "0")]
    depth: u32,
    #[arg(short, long)]
    infinite: bool,
    #[arg(short, long)]
    root: String,
}

fn do_fetch<'a>(
    name: &str,
    repo: &'a git2::Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();

    // Print out our transfer progress.
    cb.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "{name}: Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "{name}: Received {}/{} objects ({}) in {} bytes\r",
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
    println!(
        "Fetching {} for repo {name}",
        remote.name().unwrap_or("{REPO NAME FAILED}")
    );
    remote.fetch(refs, Some(&mut fo), None)?;

    // If there are local objects (we got a thin pack), then tell the user
    // how many objects we saved from having to cross the network.
    let stats = remote.stats();
    if stats.local_objects() > 0 {
        println!(
            "\r{name}: Received {}/{} objects in {} bytes (used {} local \
             objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        println!(
            "\r{name}: Received {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(repo.reference_to_annotated_commit(&fetch_head)?)
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
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an example so maybe not.
            .force(),
    ))?;
    Ok(())
}

fn normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("Merge: {} into {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;
    // Do our merge commit and set current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}

fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: git2::AnnotatedCommit<'a>,
) -> Result<(), git2::Error> {
    // 1. do a merge analysis
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    // 2. Do the appropriate merge
    if analysis.0.is_fast_forward() {
        println!("Doing a fast forward");
        // do a fast forward
        let refname = format!("refs/heads/{}", remote_branch);
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                fast_forward(repo, &mut r, &fetch_commit)?;
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
                )?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(
                    git2::build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))?;
            }
        };
    } else if analysis.0.is_normal() {
        // do a normal merge
        let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;
        normal_merge(&repo, &head_commit, &fetch_commit)?;
    } else {
        // println!("Nothing to do...");
    }
    Ok(())
}

fn run_inner(
    name: &str,
    repo: &Repository,
    remote_name: &str,
    remote_branch: &str,
) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote(remote_name)?;
    let fetch_commit = do_fetch(name, &repo, &[remote_branch], &mut remote)?;
    Ok(do_merge(&repo, &remote_branch, fetch_commit)?)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let key = std::env::var("GITHUB_AUTH").unwrap();

    let users: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let depth = if args.infinite { u32::MAX } else { args.depth };

    handle_user(args.root.into(), key, args.user, depth, users).await;

    loop {}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    name: String,
    full_name: String,
    id: u32,
    html_url: Url,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    login: String,
}

#[async_recursion]
async fn handle_user(
    root: PathBuf,
    key: String,
    user: String,
    iterations_remaining: u32,
    checked_users: Arc<Mutex<Vec<String>>>,
) {
    if checked_users.lock().await.contains(&user) {
        return;
    }
    checked_users.lock().await.push(user.clone());
    tokio::task::spawn(pull_user_repos(&root, &key, &user));

    if iterations_remaining > 0 {
        tokio::task::spawn(init_user_connections_job(
            &root,
            &key,
            &user,
            iterations_remaining,
            checked_users,
        ));
    }
}

async fn pull_user_repos(root: &PathBuf, key: &str, user: &str) {
    let repos = get_user_repos(&key, &user).await;
    println!("User {user} has {amount} repos", amount = repos.len());

    let mut tasks = vec![];
    for repo in &repos {
        tasks.push(handle_repo(repo.clone(), root.clone()));
    }
    futures::future::join_all(tasks).await;

    println!("Finished fetching user {user} repos");
}

async fn init_user_connections_job(
    root: &PathBuf,
    key: &str,
    user: &str,
    mut iterations_remaining: u32,
    checked_users: Arc<Mutex<Vec<String>>>,
) {
    iterations_remaining -= 1;

    let mut following = vec![];
    let mut followers = vec![];

    match get_user_following(key.to_string(), user.to_string()).await {
        Ok(inner_following) => {
            following.extend(inner_following);
        }
        Err(_e) => {}
    };
    match get_user_followers(key.to_string(), user.to_string()).await {
        Ok(inner_followers) => {
            followers.extend(inner_followers);
        }
        Err(_e) => {}
    };
    for f_user in following {
        tokio::task::spawn(handle_user(
            root.clone(),
            key.to_string(),
            f_user,
            iterations_remaining,
            Arc::clone(&checked_users),
        ));
    }
    for f_user in followers {
        tokio::task::spawn(handle_user(
            root.clone(),
            key.to_string(),
            f_user,
            iterations_remaining,
            Arc::clone(&checked_users),
        ));
    }
}

async fn get_user_following(
    key: String,
    user: String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("https://api.github.com/users/{user}/following"))?;
    let resp = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "git-mirror-reqwest")
        .bearer_auth(key)
        .send()
        .await?;
    println!("RESP :D {resp:?}");
    let resp: Vec<User> = resp.json().await?;
    println!("JSON :D {resp:?}");
    Ok(resp.into_iter().map(|user| user.login).collect())
}

async fn get_user_followers(
    key: String,
    user: String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("https://api.github.com/users/{user}/followers"))?;
    let resp = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "git-mirror-reqwest")
        .bearer_auth(key)
        .send()
        .await?;
    println!("RESP :D {resp:?}");
    let resp: Vec<User> = resp.json().await?;
    println!("JSON :D {resp:?}");
    Ok(resp.into_iter().map(|user| user.login).collect())
}

async fn get_user_repos(key: &str, user: &str) -> Vec<Repo> {
    let url = Url::parse(&format!("https://api.github.com/users/{user}/repos")).unwrap();
    let resp = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "git-mirror-reqwest")
        .bearer_auth(key)
        .send()
        .await
        .expect(&format!("Failed to get {user} repos"));

    let resp: Value = match resp.json().await {
        Ok(ok) => ok,
        Err(e) => {
            panic!("Failed to fetch repos for user {user}: {e}\n")
        }
    };
    let parsed: Vec<Repo> = match serde_json::from_value(resp.clone()) {
        Ok(ok) => ok,
        Err(e) => {
            panic!("Failed to fetch repos for user {user}: {e}\n{resp}")
        }
    };
    let names = parsed
        .iter()
        .map(|r| r.name.as_str())
        .collect::<Vec<&str>>();
    println!("User {user} har repos {names:?}");
    parsed
}

#[async_recursion]
async fn handle_repo(repo: Repo, mut root: PathBuf) {
    let body = async {
        let was_paused = IS_PAUSED.load(std::sync::atomic::Ordering::Relaxed);
        // Check if paused
        while IS_PAUSED.load(std::sync::atomic::Ordering::Relaxed) {
            println!("Pause :(");
            // You can yield or sleep here to avoid busy-waiting
            sleep(Duration::from_millis(10)).await;
        }
        if was_paused {
            println!("Unpaused; fetching {rname}", rname = repo.full_name)
        }
        let extension = PathBuf::from_str(&repo.full_name).unwrap();
        root.extend(&extension);
        tokio::fs::create_dir_all(&root).await.unwrap();
        let _fetched_repo = match Repository::open(&root) {
            Ok(fetched_repo) => {
                println!("Fetching {rname}", rname = repo.full_name);
                match run_inner(&repo.full_name, &fetched_repo, "origin", "master") {
                    Ok(_) => {}
                    Err(_e) => {
                        // repo is probably corrupted, just re-clone it
                        tokio::fs::remove_dir_all(root.clone()).await.unwrap();
                        handle_repo(repo, root).await;
                    }
                };
                Ok(fetched_repo)
            }
            Err(_) => {
                println!(
                    "Could not find repo {rname}, cloning",
                    rname = repo.full_name
                );
                match Repository::clone(&repo.clone_url, &root) {
                    Ok(fetched_repo) => Ok(fetched_repo),
                    Err(e) => {
                        println!("FUCK {e:?}\nRepo is {rname}", rname = repo.full_name);
                        global_pause(Duration::from_secs(1)).await;
                        handle_repo(repo, root).await;
                        Err(())
                    }
                }
            }
        };
    };
    tokio::task::spawn(body);
}

async fn global_pause(duration: Duration) {
    println!(
        "Pausing execution for {msecs}ms",
        msecs = duration.as_millis()
    );
    // Set the global timeout
    *GLOBAL_TIMEOUT.lock().await = Instant::now() + duration;

    // Set is_paused to true
    IS_PAUSED.store(true, std::sync::atomic::Ordering::Relaxed);

    // Wait for the global timeout
    while Instant::now() < *GLOBAL_TIMEOUT.lock().await {
        println!("Still paused pls wait");
        sleep(Duration::from_millis(10)).await;
    }
    IS_PAUSED.store(false, std::sync::atomic::Ordering::Relaxed);
    println!("Ok continue execution time :3");
}
