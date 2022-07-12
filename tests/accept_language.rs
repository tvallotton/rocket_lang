use rocket::{
    http::{Header, Status},
    local::asynchronous::{Client, LocalResponse},
};
use rocket_lang::*;
mod common;
use common::*;

async fn get_with<'a>(client: &'a Client, value: &'static str) -> LocalResponse<'a> {
    let mut req = client.clone().get("/");
    req.add_header(Header::new("accept-language", value));
    req.dispatch().await
}
async fn assert_lang(client: &Client, header: &'static str, lang: &str) {
    let body = get_with(client, header)
        .await
        .into_string()
        .await
        .unwrap();
    assert_eq!(body, lang);
}
async fn assert_not_acceptable(client: &Client, header: &'static str) {
    let res = get_with(client, header).await;
    let status = res.status();
    assert_eq!(
        status,
        Status::NotAcceptable,
        "content: {}",
        res.into_string()
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn accept_language_configured() {
    let mut config = Config::new();
    config[En] = 0.5;
    config[De] = 0.5;
    config[Es] = 1.0;
    let client = &configured(config).await;
    assert_lang(client, "en-US, de;q=0.2", "en").await;
    assert_lang(client, "de,es;q=0.5", "de").await;
    assert_lang(client, "de, es;q=0.6", "es").await;
    assert_lang(client, "de, es;q=0.4", "de").await;
    assert_lang(client, "en-US, de;q=0.2", "en").await;
    assert_lang(client, "es,invalid", "es").await;
    assert_not_acceptable(client, "pt,fr;q=0.3").await;
    assert_not_acceptable(client, "not a valid request").await;
    assert_not_acceptable(client, "xx").await;
    assert_not_acceptable(client, "xx;q=0.2").await;

    let mut config = Config::new();
    for &code in LangCode::ALL_CODES {
        config[code] = 1.0; 
    }
    let client = &configured(config).await;
    assert_lang(client, "en-US, de;q=0.2", "en").await;
    assert_lang(client, "de,es;q=0.5", "de").await;
    assert_lang(client, "de, es;q=0.6", "de").await;
    assert_lang(client, "de, es;q=0.4", "de").await;
    assert_lang(client, "en-US, de;q=0.2", "en").await;
    assert_lang(client, "fr;q=0.3", "fr").await;
    assert_not_acceptable(client, "not a valid request").await;
    assert_not_acceptable(client, "xx").await;
    assert_not_acceptable(client, "xx;q=0.2").await;
}

#[tokio::test]
async fn accept_language_not_configured() {
    let client = &not_configured().await;
    assert_lang(client, "en-US, de;q=0.2", "en").await;
    assert_lang(client, "de,es;q=0.5", "en").await;
    assert_lang(client, "de, es;q=0.6", "en").await;
    assert_lang(client, "de, es;q=0.4", "en").await;
    assert_lang(client, "en-US, de;q=0.2", "en").await;
    assert_lang(client, "fr;q=0.3", "en").await;
    assert_lang(client, "not a valid request", "en").await;
    assert_lang(client, "xx", "en").await;
    assert_lang(client, "xx;q=0.2", "en").await;
}
