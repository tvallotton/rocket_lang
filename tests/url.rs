mod common;
use common::*;
use rocket::http::Status;
async fn test_config(url: &str, lang: &str, config: Config) {
    let body = configured(config)
        .await
        .get(url)
        .dispatch()
        .await
        .into_string()
        .await
        .unwrap();
    assert_eq!(body, lang);
}
#[tokio::test]
async fn url_minus_one() {
    let config = Config::new().url(-1);
    test_config("/index/path/es", "es", config).await;
}

#[tokio::test]
async fn url_negative_index() {
    let config = Config::new().url(-2);
    test_config("/index/fr/segment", "fr", config).await;
}

#[tokio::test]
async fn url_positive_index() {
    let config = Config::new().url(0);
    test_config("/de/some/path", "de", config).await;

    let config = Config::new().url(1);
    test_config("/some/pt/path", "pt", config).await;
}
#[tokio::test]
async fn url_not_a_lang_code() {
    let status = configured(Config::new().url(0))
        .await
        .get("/fail")
        .dispatch()
        .await
        .status();
    assert!(status == Status::NotFound);
}

#[tokio::test]
async fn url_not_long_enough_positive() {
    let status = configured(Config::new().url(3))
        .await
        .get("/fail")
        .dispatch()
        .await
        .status();
    assert!(status == Status::NotFound);
}
#[tokio::test]
async fn url_not_long_enough_minus_one() {
    let status = configured(Config::new().url(-1))
        .await
        .get("/")
        .dispatch()
        .await
        .status();
    assert!(status == Status::NotFound);
}
#[tokio::test]
async fn url_not_long_enough_negative() {
    let status = configured(Config::new().url(-4))
        .await
        .get("/")
        .dispatch()
        .await
        .status();
    assert!(status == Status::NotFound);
}
