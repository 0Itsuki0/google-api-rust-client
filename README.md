# Google API Client
An unoffical client library for Google APIs. <br>
crate.io: [google-api-rust-client-unoffical](https://crates.io/crates/google-api-rust-client-unoffical)
<br>

## Installation
Run the following Cargo command in your project directory:
```
cargo add google-api-rust-client-unoffical
```
Or add the following line to your Cargo.toml:
```
google-api-rust-client-unoffical = "0.1.3"
```
<br>

## Authorization
Obtain Service Account credentials or API Key from APIs & Services > Credentials in the Google Cloud Console.


### Service Account
#### Create Credentials From JSON File
```
let filepath: PathBuf = PathBuf::from_str("credentials.json")?;
let credentials = ServiceAccountCredentials::from_service_account_file(filepath)?;
```

#### Create Credentials From JSON String
```
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
```
#### Setting Scopes
```
let scoped_credentials = credentials.with_scopes(vec!["https://www.googleapis.com/auth/cloud-translation"]);
```

#### Setting Subject
```
let subjected_crentials = credentials.with_subject("itsuki@example.com");
```

#### Fetching Access Token Directly
```
let token = credentials.get_access_token().await?;
```

#### Using Credentials with Services
```
let mut translation_service = TranslateService::new_with_credentials(credentials);
let response = translation_service.list_languages(Some("ja"), None).await?;
```


### API KEY
To use API keys, pass them in when creating service objects.
```
let api_key = env::var("API_KEY")?;
let mut translation_service = TranslateService::new_with_api_key(api_key);
let response = translation_service.list_languages(Some("ja"), None).await?;
```
<br>

Refer to [auth.rs](/examples/auth.rs) for authorization example.

<br>

## Services

### Cloud Translation Basic Edition (v2)
Supported APIs
- [Translate](https://cloud.google.com/translate/docs/reference/rest/v2/translate): Translates input text, returning translated text.
- [Detect](https://cloud.google.com/translate/docs/reference/rest/v2/detect): Detects the language of texts.
- [Languages](https://cloud.google.com/translate/docs/reference/rest/v2/languages): Returns a list of supported languages for translation.

<br>

Refer to [translate_service.rs](/examples/translate_service.rs) for usage examples.<br>


### Other Services Under Construction!