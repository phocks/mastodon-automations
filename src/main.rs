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
    mastodon
        .follows_me()
        .await?
        .items_iter()
        .for_each(|account| async move {
            println!("{}", account.acct);
        })
        .await;

    Ok(())
}
