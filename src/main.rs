mod instance;

use std::env;
use instance::instance;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL as an environment variable.");
        return
    };

    match instance(url.as_str()).await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

