use momento::cache::{configurations, CreateCacheResponse};
use momento::{CacheClient, CredentialProvider, MomentoError};
use std::time::Duration;

const CACHE_NAME: &str = "momento-rust-test";
const KEY: &str = "cache_key";
const VALUE: &str = "Hello, Momento!";

#[tokio::main]
async fn main() -> Result<(), MomentoError> {
    let cache_client = CacheClient::builder()
        .default_ttl(Duration::from_secs(60))
        .configuration(configurations::Laptop::latest())
        .credential_provider(CredentialProvider::from_env_var(
            "MOMENTO_API_KEY".to_string(),
        )?)
        .build()?;

    match cache_client.create_cache(CACHE_NAME).await? {
        CreateCacheResponse::Created => println!("Cache {} created", CACHE_NAME),
        CreateCacheResponse::AlreadyExists => println!("Cache {} already exists", CACHE_NAME),
    }

    cache_client.set(CACHE_NAME, KEY, VALUE).await?;
    println!("Value stored");

    let response = cache_client.get(CACHE_NAME, KEY).await?;
    let item: String = response.try_into().expect("I stored a string!");
    println!("Cache value: {}", item);

    cache_client.delete_cache(CACHE_NAME).await?;
    println!("Cache deleted");

    Ok(())
}
