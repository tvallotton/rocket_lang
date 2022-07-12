use crate::*;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use rocket::Request;

static PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:^|,| )(\w{1,3})(?:-\w{1,3})? ?(?:;q=([\d\.]+))?").unwrap());

fn accept_language<'a>(req: &'a Request<'_>) -> &'a str {
    req.headers()
        .get("Accept-Language")
        .next()
        .unwrap_or("en")
}

fn lang_from_capture(capt: &Captures) -> Option<LangCode> {
    capt.iter()
        .flatten()
        .map(|m| m.as_str())
        .map(|m| m.parse())
        .map(|m| m.ok())
        .nth(1)?
}
fn quality_from_capture(capt: &Captures) -> f32 {
    capt.iter()
        .nth(3)
        .or_else(|| capt.iter().nth(2))
        .flatten()
        .map(|m| m.as_str())
        .map(|m| m.parse())
        .and_then(|r| r.ok())
        .unwrap_or(1.0)
}

fn from_regex_capture(cap: Captures) -> Option<(LangCode, f32)> {
    let lang = lang_from_capture(&cap)?;
    let q = quality_from_capture(&cap);
    Some((lang, q))
}

pub(crate) fn languages(text: &'_ str) -> impl Iterator<Item = (LangCode, f32)> + '_ {
    PATTERN
        .captures_iter(text)
        .flat_map(from_regex_capture)
}

struct Decider<'a> {
    lang: Option<LangCode>,
    q: Option<f32>,
    config: &'a Config,
}

impl<'a> Decider<'a> {
    fn new(config: &'a Config) -> Self {
        Self {
            lang: None,
            q: None,
            config,
        }
    }
    fn is_none(&self) -> bool {
        self.lang.is_none()
    }
    fn compare(&mut self, lang2: LangCode, qclient2: f32) {
        let lang1 = self.lang.unwrap();
        let qclient1 = self.q.unwrap();
        let qserver1 = self.config[lang1];
        let qserver2 = self.config[lang2];
        if (qserver1 - qserver2) / qserver1 < (qclient2 - qclient1) / qclient2 {
            self.lang = Some(lang2);
            self.q = Some(qclient2)
        }
    }
    fn add_preference(&mut self, lang: LangCode, q: f32) {
        if self.config[lang] == 0.0 || self.config[lang].is_nan() {
            return;
        }
        if self.is_none() {
            self.lang = Some(lang);
            self.q = Some(q);
        } else {
            self.compare(lang, q);
        }
    }
    fn result(&self) -> Result<LangCode, Error> {
        self.lang
            .ok_or(Error::NotAcceptable)
    }
}

pub(crate) fn with_config(req: &Request, config: &Config) -> Result<LangCode, Error> {
    let header = accept_language(req);
    let mut decider = Decider::new(config);
    for (lang, q) in languages(header) {
        decider.add_preference(lang, q);
    }
    decider.result()
}
