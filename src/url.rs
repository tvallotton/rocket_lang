#![warn(dead_code)]
use crate::error::Error;
use rocket::Request;

use crate::LangCode;

fn minus_one_optimization(req: &Request) -> Option<LangCode> {
    return req
        .uri()
        .path()
        .segments()
        .last()?
        .parse()
        .ok();
}

fn parse_negative(req: &Request, pos: i32) -> Option<LangCode> {
    if pos == -1 {
        return minus_one_optimization(req);
    }
    req.uri()
        .path()
        .segments()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .skip((-1 - pos) as usize)
        .next()?
        .parse()
        .ok()
}
fn parse_positive(req: &Request, pos: i32) -> Option<LangCode> {
    req.uri()
        .path()
        .segments()
        .skip(pos as usize)
        .next()?
        .parse()
        .ok()
}

fn parse(req: &Request, pos: i32) -> Option<LangCode> {
    if pos.is_negative() {
        parse_negative(req, pos)
    } else {
        parse_positive(req, pos)
    }
}

pub(crate) fn get(req: &Request<'_>, pos: i32) -> Result<LangCode, Error> {
    parse(req, pos).ok_or(Error::NotFound)
}
