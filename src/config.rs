use crate::*;
use rocket::{
    async_trait,
    fairing::{Fairing, Info, Kind},
    Build, Data, Request, Rocket, State,
};
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
    sync::Arc,
};

// type alias for reduced verbosity
type CloneFn = Arc<dyn Fn(&Request) -> Result<LangCode, Error> + 'static + Send + Sync>;

/// This struct allows for customization of `LangCode`'s
/// behavior.
/// The precedence for every configuration is:
///   1. custom closure
///   2. from url
///   3. accept language header
///   4. wildcard
///
/// If none of these are able to produce an Ok value, an error is returned.
/// Note that returning errors is discouraged, as it may lead to a poor user experience.
///
///
/// ## Custom closure
/// This configuration has the biggest precedence.
/// There is full access to the incoming request.
/// ```
/// let config = Config::new().custom(|request: &Request| {
///     Ok(lang_from_request(request))
/// });
/// ```
/// ## Url
/// The url method can be used to specify which segment
/// should be interpreted as a language code. Negative indexes
/// can be used to refer to positions relative to the last segment.
/// Thus -1 corresponds to the last segment, -2 the second to last, and so on.
///
/// ### examples
/// ```rust
/// // we use -1 to specify that the last segment is our language code
/// let config = Config::new().url(-1);
/// // we have to specify which language we want on the handler
/// #[get("/index/en")
/// fn english_language() {
///    /* ... */
/// }
/// // we can handle all languages at once with a wildcard segment.
/// // if we visit with an invalid language code ("/index/not-a-lang-code/"), an error is returned.
/// #[get("/index/<_>", rank = 2)
/// fn any_language() {
///    /* ... */
/// }
/// ```
///
/// ## Accept Language
/// The accept language header qualities can be set by indexing into the config struct.
/// By default, all values are set to 0.0. These values should correspond to a
/// number between 0.0 and 1.0 specifying the quality of that language support of your site.
/// ```rust
/// let config = Config::new();
/// config[En] = 0.3;
/// config[Ar] = 1.0;
/// ```
///
/// ## Wildcard
/// The wildcard will be used to create a value if none of the previous attempts succeeded.
/// Note that wildcards are useful for single language applications, but they may not scale as well as url resolution.
/// By default the wildcard is set to `None`.
///
/// ```rust
/// let config = Config::new().url(1).wildcard(Es);
/// ```

#[derive(Clone)]
pub struct Config {
    pub wildcard: Option<LangCode>,
    pub(crate) accept_language: HashMap<LangCode, f32>,
    pub(crate) url: Option<i32>,
    pub(crate) custom: Option<CloneFn>,
}

impl Config {
    /// The wildcard is used as a last resort if all other options failed.
    pub fn wildcard(mut self, lang: LangCode) -> Self {
        self.wildcard = Some(lang);
        self
    }
    /// Takes the Config structure by value and returns
    /// a new one with the url configuration set.
    /// The position parameter determines which path segment
    /// will be interpreted as a language code.
    /// Negative positions will be interpreted as being relative
    /// to the last path segment.
    pub fn url(mut self, position: i32) -> Self {
        self.url = Some(position);
        self
    }

    /// Constructs a new configuration object.
    pub fn new() -> Self {
        Self::default()
    }
    /// Used to specify a custom language resolution method.
    pub fn custom<F>(self, f: F) -> Self
    where
        F: Fn(&Request) -> Result<LangCode, Error> + 'static + Send + Sync,
    {
        Self {
            custom: Some(Arc::new(f)),
            ..self
        }
    }
    /// The errors with be grater preference should be

    pub(crate) fn choose(&self, req: &Request<'_>) -> Result<LangCode, Error> {
        self.from_custom(req)
            .or_else(|e1| {
                self.from_url(req)
                    .map_err(|e2| e1.or(e2))
            })
            .or_else(|e1| {
                self.from_lang_header(req)
                    .map_err(|e2| e1.or(Some(e2)))
            })
            .or_else(|err| {
                if let Some(val) = self.wildcard {
                    return Ok(val);
                }
                Err(err)
            })
            .map_err(Option::unwrap)
    }

    fn from_custom(&self, req: &Request) -> Result<LangCode, Option<Error>> {
        if let Some(custom) = self.custom.as_ref() {
            return custom(req).map_err(Some);
        }
        Err(None)
    }

    fn from_url(&self, req: &Request) -> Result<LangCode, Option<Error>> {
        if let Some(pos) = self.url {
            return crate::url::get(req, pos).map_err(Some);
        }
        Err(None)
    }

    fn from_lang_header(&self, req: &Request) -> Result<LangCode, Error> {
        crate::accept_language::with_config(req, self)
    }
}

pub(crate) struct PrivConfig(Config);

impl Default for Config {
    fn default() -> Self {
        let mut config = Config {
            wildcard: None,
            url: None,
            accept_language: HashMap::with_capacity(LangCode::ALL_CODES.len()),
            custom: None,
        };
        for lang in LangCode::ALL_CODES {
            config
                .accept_language
                .insert(*lang, 0.0);
        }
        config
    }
}

#[async_trait]
impl Fairing for Config {
    fn info(&self) -> Info {
        Info {
            name: "Language configuration",
            kind: Kind::Ignite | Kind::Request,
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        Ok(rocket.manage(PrivConfig(self.clone())))
    }
    // this will get executed before routing the request
    // so retrieving the `langCode` guard
    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        if let Outcome::Success(config) = req
            .guard::<&State<PrivConfig>>()
            .await
        {
            req.local_cache(|| config.0.choose(req));
        }
    }
}
// async fn on_request(req: &mut Request<'_>, _data: &mut Data<'_>) {
//     match req.guard::<&State<PrivConfig>>().await {
//         Outcome::Success(config) => config.0.choose(req)?,
//         _ => crate::accept_language::without_config(req)?,
//     }

//     if let Outcome::Success(code) = req.guard::<LangCode>().await {
//         req.local_cache(|| code);
//     }
// }

impl Index<LangCode> for Config {
    type Output = f32;
    fn index(&self, index: LangCode) -> &Self::Output {
        &self.accept_language[&index]
    }
}

impl IndexMut<LangCode> for Config {
    fn index_mut(&mut self, index: LangCode) -> &mut Self::Output {
        self.accept_language
            .get_mut(&index)
            .unwrap()
    }
}

// fn parse_with_config(
//     text: &str,
//     map: &impl Index<LangCode, Output = f32>,
// ) -> Result<Option<LangCode>, Error> {
//     if text == "*" {
//         return Ok(None);
//     }

//     for cap in PATTERN.captures(text) {
//         if let Some((lang, q)) = from_regex_capture(cap, map) {
//             if q <= map[lang] {
//                 return Some(lang);
//             }
//         }
//     }
//     Err(Error::AcceptLanguageParse)
// }
