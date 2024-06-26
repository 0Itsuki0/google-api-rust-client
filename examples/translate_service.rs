use dotenvy::dotenv;
use google_api_rust_client_unoffical::{auth::service_account::ServiceAccountCredentials, services::translate_service::TranslateService};
use std::{env, path::PathBuf, str::FromStr, vec};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // translate
    translate().await?;

    // list languages
    list_languages().await?;

    // detect language
    detect_language().await?;

    Ok(())
}


async fn translate() -> Result<()> {
    dotenv().ok();

    // api auth
    let api_key = env::var("API_KEY")?;
    let mut translation_service = TranslateService::new_with_api_key(api_key);
    let response = translation_service.translate(vec!["test"], "ja").await?;
    println!("response: {}", serde_json::to_string(&response)?);

    // service account auth
    let filepath: PathBuf = PathBuf::from_str("credentials.json")?;
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)?;
    let mut translation_service = TranslateService::new_with_credentials(credentials);
    let response = translation_service.translate(vec!["test"], "ja").await?;
    println!("response: {}", serde_json::to_string(&response)?);
    let response_with_params = translation_service.translate_with_params(vec!["test2"], "ja", Some("text"), Some("en"), Some("base")).await?;
    println!("response_with_params: {}", serde_json::to_string(&response_with_params)?);

    Ok(())
}


async fn list_languages() -> Result<()> {
    dotenv().ok();

    // api auth
    let api_key = env::var("API_KEY")?;
    let mut translation_service = TranslateService::new_with_api_key(api_key);
    let response = translation_service.list_languages(Some("ja"), None).await?;
    println!("response: {}", serde_json::to_string(&response)?);

    // service account auth
    let filepath: PathBuf = PathBuf::from_str("credentials.json")?;
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)?;
    let mut translation_service = TranslateService::new_with_credentials(credentials);
    let response = translation_service.list_languages(Some("ja"), None).await?;
    println!("response: {}", serde_json::to_string(&response)?);

    Ok(())
}


async fn detect_language() -> Result<()> {
    dotenv().ok();

    // api auth
    let api_key = env::var("API_KEY")?;
    let mut translation_service = TranslateService::new_with_api_key(api_key);
    let response = translation_service.detect_language(vec!["test", "テスト"]).await?;
    println!("response: {}", serde_json::to_string(&response)?);

    // service account auth
    let filepath: PathBuf = PathBuf::from_str("credentials.json")?;
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)?;
    let mut translation_service = TranslateService::new_with_credentials(credentials);
    let response = translation_service.detect_language(vec!["test", "テスト"]).await?;
    println!("response: {}", serde_json::to_string(&response)?);

    Ok(())
}
