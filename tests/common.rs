use rocket::local::asynchronous::Client;
use rocket::{get, routes};
pub use rocket_lang::Config;
pub use rocket_lang::*;

#[get("/<_>/<_>/<_>")]
fn index(lang: LangCode) -> &'static str {
    lang.as_str()
}

#[get("/fail")]
fn fails(lang: Result<LangCode, Error>) -> Result<&'static str, Error> {
    println!("{lang:?}"); 
    Ok(lang?.as_str())
}
#[get("/")]
fn nothing(lang: LangCode) -> &'static str {
    lang.as_str()
}

pub async fn configured(config: Config) -> Client {
    let rocket = rocket::build()
        .mount("/namespaced", routes![])
        .mount("/", routes![index, fails, nothing])
        .attach(config);
    Client::tracked(rocket)
        .await
        .unwrap()
}
#[allow(dead_code)]
pub async fn not_configured() -> Client {
    let rocket = rocket::build().mount("/", routes![index, fails, nothing]);
    Client::tracked(rocket)
        .await
        .unwrap()
}
