mod common;
use common::*;
use rocket::http::{Header, Status};

async fn get_mixed(config: Config, path: &str, header: &'static str) -> (String, Status) {
    let client = configured(config).await;
    let mut request = client.get(path);
    request.add_header(Header::new("accept-language", header));
    let response = request.dispatch().await;
    let s = response.status();
    (
        response
            .into_string()
            .await
            .unwrap(),
        s,
    )
}

async fn assert_mixed(config: Config, path: &'static str, header: &'static str, lang: &str) {
    let body = get_mixed(config, path, header)
        .await
        .0;
    assert_eq!(body, lang);
}

async fn assert_status(config: Config, path: &str, header: &'static str, status: Status) {
    let s = get_mixed(config, path, header)
        .await
        .1;
    assert_eq!(s, status);
}

#[tokio::test]
async fn mixed_url_accept_language() {
    let mut config = Config::new().url(-1);
    config[Es] = 1.0;
    assert_mixed(config.clone(), "/some/bad/path", "en,es;q=0.1", "es").await;
    assert_mixed(config.clone(), "/some/bad/la", "en,es;q=0.1", "la").await;
    assert_status(config, "/some/bad/path", "en", Status::NotFound).await;
}

#[tokio::test]
async fn mixed_url_wildcard() {
    let mut config = Config::new()
        .url(-1)
        .wildcard(De);
    config[Es] = 1.0;
    assert_mixed(config.clone(), "/some/bad/path", "en", "de").await;
    assert_mixed(config.clone(), "/some/bad/path", "es", "es").await;
    assert_mixed(config.clone(), "/some-good/path/pt", "es", "pt").await;
    assert_status(
        config.clone(),
        "/some/bad/path",
        "not a valid header",
        Status::Ok,
    )
    .await;
    assert_status(config, "/some/bad/path", "", Status::Ok).await;
}

#[tokio::test]
async fn mixed_custom_url() {
    let mut config = Config::new()
        .url(-1)
        .wildcard(De)
        .custom(|_| Err(Error::NotFound));

    config[Es] = 1.0;
    assert_mixed(config.clone(), "/some/bad/path", "en", "de").await;
    assert_mixed(config.clone(), "/some/bad/path", "en", "de").await;
    assert_mixed(config.clone(), "/some/bad/path", "es", "es").await;
    assert_mixed(config.clone(), "/some-good/path/pt", "es", "pt").await;

    let config = Config::new().custom(|_| Ok(La));

    assert_mixed(config.clone(), "/some/bad/path", "en", "la").await;
    assert_mixed(config.clone(), "/some/bad/path", "en", "la").await;
    assert_mixed(config.clone(), "/some/bad/path", "es", "la").await;
    assert_mixed(config.clone(), "/some-good/path/pt", "es", "la").await;
}
