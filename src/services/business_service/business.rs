use super::BusinessService;
use anyhow::{anyhow, bail, Result};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{header::HeaderMap, Client, Url};
use serde::Deserialize;

// Translates text into the target language.
// Target must be an ISO 639-1 language code.
// See https://g.co/cloud/translate/v2/translate-reference#supported_languages
// https://cloud.google.com/translate/docs/reference/rest/v2/translate
// https://cloud.google.com/translate/docs/basic/translating-text#translate_translate_text-drest

impl BusinessService {
    pub async fn accounts(self) -> Result<String> {
        let base_url = Url::parse(&format!(
            "https://mybusinessbusinessinformation.googleapis.com/v1/accounts/{}",
            ""
        ))?;
        let mut headers = HeaderMap::new();

        if let Some(api_key) = self.api_key {
            headers.insert("X-goog-api-key", HeaderValue::from_str(&api_key)?);
        } else if let Some(mut credentials) = self.service_account_credentials {
            let token = credentials.get_access_token().await?;
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        } else {
            bail!("Unknown Auth Method!")
        };

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        //headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
        let response = Client::new().get(base_url).headers(headers).send().await?;

        let resp: serde_json::Value = response.json().await?;

        let accounts: Vec<_> = resp.get("accounts").unwrap().as_array().unwrap().to_vec();
        if accounts.len() == 0 {
            return Err(anyhow!("no accounts, something went wrong!"));
        }

        let my_account = &accounts[0];
        let acc_id = my_account
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
            .split("/")
            .collect::<Vec<_>>()[1]
            .to_string();
        Ok(acc_id)
    }
    pub async fn locations(self, account_id: &str) -> Result<Vec<Page>> {
        let mut base_url = Url::parse(&format!(
            //locations
            "https://mybusinessbusinessinformation.googleapis.com/v1/accounts/{}/locations?readMask=name,title,storeCode",
            //location
            //"https://mybusinessbusinessinformation.googleapis.com/v1/accounts/{}/locations/9288306414684957101",
            // reviews
            // "https://mybusiness.googleapis.com/v4/accounts/{}/locations/5031657144081502405/reviews",
            // review
            // "https://mybusiness.googleapis.com/v4/accounts/{}/locations/5031657144081502405/reviews/AbFvOqkfvxRZQjTHdDzmv40njTkoJHGEV7HRvzlp8hYx4ZHFs_6gjDKuPDjgzmKmRZBfjVPTFAgtkQ",
           // "" 
            "-"
        ))?;
        let mut headers = HeaderMap::new();

        if let Some(api_key) = self.api_key {
            headers.insert("X-goog-api-key", HeaderValue::from_str(&api_key)?);
        } else if let Some(mut credentials) = self.service_account_credentials {
            let token = credentials.get_access_token().await?;
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        } else {
            bail!("Unknown Auth Method!")
        };

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let mut result: Vec<Page> = vec![];
        loop {
            let response = Client::new()
                .get(base_url.clone())
                .headers(headers.clone())
                .send()
                .await?;
            let resp: serde_json::Value = response.json().await?;
            let val_pages = resp.get("locations").unwrap().as_array().unwrap().clone();
            let pages: Vec<Page> = val_pages
                .iter()
                .map(|v| serde_json::from_value(v.clone()).unwrap())
                .collect();
            result.extend(pages);

            if let Some(next_page_token) = resp["nextPageToken"].as_str() {
                base_url = Url::parse(&format!(
                    "{}&pageToken={}",
                    base_url.clone().as_str(),
                    next_page_token
                ))?;
            } else {
                break;
            }
        }

        //println!("{:#?}", result);

        Ok(result)
    }
    pub async fn get_admins(self, locations: &Vec<Page>) -> Result<Vec<PageAdmins>> {
        let mut headers = HeaderMap::new();

        if let Some(api_key) = self.api_key {
            headers.insert("X-goog-api-key", HeaderValue::from_str(&api_key)?);
        } else if let Some(mut credentials) = self.service_account_credentials {
            let token = credentials.get_access_token().await?;
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
        } else {
            bail!("Unknown Auth Method!")
        };

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let mut results: Vec<PageAdmins> = vec![];
        for location in locations {
            let base_url = Url::parse(&format!(
                "https://mybusinessaccountmanagement.googleapis.com/v1/{}/admins",
                location.name
            ))?;
            let response = Client::new()
                .get(base_url)
                .headers(headers.clone())
                .send()
                .await?;
            let resp: serde_json::Value = response.json().await?;
            let admin_count = resp.get("admins").unwrap().as_array().unwrap().len();
            let resp = PageAdmins {
                store_code: location.store_code.clone(),
                page_name: location.name.clone(),
                page_title: location.title.clone(),
                admin_count,
            };

            results.push(resp);
        }

        Ok(results)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Page {
    pub name: String,
    pub title: String,
    #[serde(rename = "storeCode")]
    pub store_code: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct PageAdmins {
    pub page_name: String,
    pub page_title: String,
    #[serde(rename = "storeCode")]
    pub store_code: String,
    pub admin_count: usize,
}
