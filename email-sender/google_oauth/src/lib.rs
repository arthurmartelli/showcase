use common::prelude::*;
use google_sheets4::oauth2::{self, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

pub async fn get_auth() -> Result<Auth, oauth2::Error> {
    let secret = oauth2::read_application_secret("./secrets/clientsecret.json")
        .await
        .expect("./secrets/clientsecret.json");

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("./secrets/tokencache.json")
        .build()
        .await
        .unwrap();

    Ok(auth)
}

pub async fn get_user_info(token: &AccessToken, param: &str) -> Result<String, DynError> {
    // https://www.googleapis.com/oauth2/v1/userinfo?access_token={token}

    let url = format!(
        "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
        token.as_str()
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    let email = json::parse(body.as_str())?[param].to_string();

    Ok(email)
}
