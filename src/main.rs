mod instance;

use instance::{following, instance};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL as an environment variable.");
        return;
    };

    match instance(url.as_str()).await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    let following_future = following(url.as_str(), "108274904351853384");
    let response = following_future.await;

    match response {
        Ok(response) => {
            for account in response {
                println!("{:#?}", account);
            }
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
