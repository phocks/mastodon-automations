#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;
use mastodon_async::Result;

#[tokio::main] // requires `features = ["mt"]
async fn main() -> Result<()> {
    run().await
}

#[cfg(feature = "toml")]
async fn run() -> Result<()> {
    use futures::StreamExt;

    let mastodon = register::get_mastodon_data().await?;

    let followed_by_me = mastodon.followed_by_me().await?;
    let follows_me = mastodon.follows_me().await?;

    let followed_by_me_iter = followed_by_me.items_iter();
    let follows_me_iter = follows_me.items_iter();

    let fbm: Vec<_> = followed_by_me_iter.collect().await;
    let fm: Vec<_> = follows_me_iter.collect().await;

    // Compare the two lists .id and get the ids in a Vec of the difference
    let mut diff = Vec::new();
    for i in fbm.iter() {
        if !fm.contains(i) {
            diff.push(i);
        }
    }

    // Print the length of the difference
    println!("{} users in diff", diff.len());

    // Get those users that I follow but don't follow me
    let mut diff2 = Vec::new();
    for i in fm.iter() {
        if !fbm.contains(i) {
            diff2.push(i);
        }
    }

    // Print the length of the difference
    println!("{} users in diff2", diff2.len());

    // Unfollow those users that I follow but don't follow me
    for i in diff.iter() {
        println!("Unfollowing {}", i.acct);
        let unfollow = mastodon.unfollow(&i.id).await?;
        println!("{:?}", unfollow);

        // Sleep for 1 second
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
