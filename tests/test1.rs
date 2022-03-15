use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;
use rocket::{Build, Rocket};

use LangCode::Es;
#[macro_use]
extern crate rocket;

use rocket_lang::{Config, Error, LangCode};

#[get("/<_>/<_>/<_>")]
fn index(lang: LangCode) -> &'static str {
    lang.as_str()
}
#[get("/fail")]
fn fails(lang: LangCode) -> &'static str {
    lang.as_str()
}

async fn configured(config: Config) -> Client {
    let rocket = rocket::build()
        .mount("/", routes![index, fails])
        .attach(config);
    Client::tracked(rocket)
        .await
        .unwrap()
}
async fn not_configured() -> Client {
    let rocket = rocket::build().mount("/", routes![index, fails]);

    Client::tracked(rocket)
        .await
        .unwrap()
}
async fn test_config(url: &str, lang: &str, config: Config) {
    let body = configured(config)
        .await
        .get(url)
        .dispatch()
        .await
        .into_string()
        .await
        .unwrap();
    assert!(body == lang);
}

// #[tokio::test]
// async fn url_minus_one() {
//     let config = Config::new().url(-1);
//     test_config("/index/path/es", "es", config).await;
// }

// #[tokio::test]
// async fn negative_url() {
//     let config = Config::new().url(-2);
//     test_config("/index/fr/segment", "fr", config).await;
// }

// #[tokio::test]
// async fn positive_url() {
//     let config = Config::new().url(0);
//     test_config("/de/some/path", "de", config).await;

//     let config = Config::new().url(1);
//     test_config("/some/pt/path", "pt", config).await;
// }
// #[tokio::test]
// async fn url_failure() {
//     let config = Config::new().url(0);

//     let rocket = rocket::build()
//         .mount("/", routes![index, fails])
//         .attach(config);

//     let status = Client::tracked(rocket)
//         .await
//         .unwrap()
//         .get("/fail")
//         .dispatch()
//         .await
//         .status();
//     assert!(status == Status::NotFound);
// }

#[tokio::test]
async fn wildcard() {
    let config = Config::new().wildcard(Es);
    test_config("/some/other/path", "es", config).await;
}
#[tokio::test]
async fn test_custom() {
    let config = Config::new().custom(|_req| Ok(LangCode::Om));
    test_config("/some/other/path", "om", config).await;
}

#[tokio::test]
async fn test_failed_custom() {
    let config = Config::new()
        .custom(|_req| Err(Error::NotAcceptable))
        .url(-1);
    test_config("/some/other/es", "es", config).await;
}

#[tokio::test]
async fn _accept_header1() {
    let mut config = Config::new();
    config[LangCode::En] = 0.5;
    config[LangCode::De] = 0.5;
    config[LangCode::Es] = 1.0;
    let rocket = rocket::build()
        .mount("/", routes![index])
        .attach(config);
    let client = Client::tracked(rocket)
        .await
        .unwrap();

    let mut req = client.get("/some/path/index.html");
    req.add_header(Header::new("accept-language", "en-US, de;q=0.2"));
    let res = req
        .dispatch()
        .await
        .into_string()
        .await
        .unwrap();
    assert_eq!(res, "en");

    let mut req = client.get("/some/path/index.html");
    req.add_header(Header::new("accept-language", "de, es;q=0.5"));
    let res = req
        .dispatch()
        .await
        .into_string()
        .await
        .unwrap();
    assert_eq!(res, "de");
    let mut req = client.get("/some/path/index.html");
    req.add_header(Header::new("accept-language", "de, es;q=0.6"));
    let res = req
        .dispatch()
        .await
        .into_string()
        .await
        .unwrap();
    assert_eq!(res, "es");
    let mut req = client.get("/some/path/index.html");
    req.add_header(Header::new("accept-language", "de, es;q=0.4"));
    let res = req
        .dispatch()
        .await
        .into_string()
        .await
        .unwrap();
    assert_eq!(res, "de");

    let mut req = client.get("/some/path/index.html");
    req.add_header(Header::new("accept-language", "not a valid header"));
    let status = req.dispatch().await.status();
    assert!(status == Status::NotAcceptable);
}
#[tokio::test]
async fn accept_header_without_config() {
    let rocket = rocket::build().mount("/", routes![index]);

    let client = Client::tracked(rocket)
        .await
        .unwrap();
    let mut req = client.get("/some/path/index.html");
    req.add_header(Header::new("accept-language", "de, es;q=0.4"));
}
