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
    let total = req
        .uri()
        .path()
        .segments()
        .count();

    req.uri()
        .path()
        .segments()
        .nth(total.checked_sub(pos.abs() as usize)?)
        .unwrap() // we can unwrap this because the checked subtraction
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
