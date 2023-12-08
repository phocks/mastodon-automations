use megalodon::{entities, error, generator, SNS};


pub async fn instance(url: &str) -> Result<entities::Instance, error::Error> {
  let client = generator(SNS::Mastodon, url.to_string(), None, None);
  let res = client.get_instance().await?;
  Ok(res.json())
}