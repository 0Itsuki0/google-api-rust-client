use google_api_rust_client_unoffical::auth::service_account::ServiceAccountCredentials;
use std::{path::PathBuf, str::FromStr};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {

    // service account credentials from file
    let filepath: PathBuf = PathBuf::from_str("credentials.json")?;
    let credentials = ServiceAccountCredentials::from_service_account_file(filepath)?;
    let scoped_credentials = credentials.with_scopes(vec!["https://www.googleapis.com/auth/cloud-translation"]);
    let mut subjected_crentials = scoped_credentials.with_subject("itsuki@example.com");
    let token = subjected_crentials.get_access_token().await?;
    println!("token: {}", token);

    // service account credentials from json
    let credentials_json = serde_json::json!({
        "type": "service_account",
        "project_id": "xxx",
        "private_key_id": "xxx",
        "private_key": "-----BEGIN PRIVATE KEY-----\nsome_key\n-----END PRIVATE KEY-----\n",
        "client_email": "xxx@example.com",
        "client_id": "xxx",
        "auth_uri": "https://accounts.google.com/o/oauth2/auth",
        "token_uri": "https://oauth2.googleapis.com/token",
        "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
        "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/xxx",
        "universe_domain": "googleapis.com"
    }).to_string();
    let credentials = ServiceAccountCredentials::from_service_account_info(credentials_json)?;
    let scoped_credentials = credentials.with_scopes(vec!["https://www.googleapis.com/auth/cloud-translation"]);
    let mut subjected_crentials = scoped_credentials.with_subject("itsuki@example.com");
    let token = subjected_crentials.get_access_token().await?;
    println!("token: {}", token);

    Ok(())
}
