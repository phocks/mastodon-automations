use mastodon_async::helpers::toml; // requires `features = ["toml"]`
use mastodon_async::prelude::*;
use mastodon_async::{helpers::cli, Result};

#[tokio::main] // requires `features = ["mt"]
async fn main() -> Result<()> {
    let mastodon = if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Mastodon::from(data)
    } else {
        register().await?
    };

    let you = mastodon.verify_credentials().await?;

    println!("{:#?}", you);

    Ok(())
}

async fn register() -> Result<Mastodon> {
    let registration = Registration::new("https://phocks.eu.org")
        .client_name("mastodon-async-examples")
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
}

// mod mega;

// use mega::following;
// use std::env;

// #[tokio::main]
// async fn main() {
//     env_logger::init();

//     let Ok(url) = env::var("MASTODON_URL") else {
//         println!("Specify MASTODON_URL as an environment variable.");
//         return;
//     };

//     // match instance(url.as_str()).await {
//     //     Ok(response) => {
//     //         println!("{:#?}", response);
//     //     }
//     //     Err(err) => {
//     //         println!("{:#?}", err);
//     //     }
//     // }

//     let following_future = following(url.as_str(), "108274904351853384");
//     let response = following_future.await;

//     match response {
//         Ok(response) => {
//             for account in response {
//                 println!("{:#?}", account);
//             }
//         }
//         Err(err) => {
//             println!("{:#?}", err);
//         }
//     }
// }
