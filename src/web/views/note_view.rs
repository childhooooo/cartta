use pulldown_cmark::{html, Parser};
use crate::contexts::cms::{*, note::Note, tag::Tag};
use std::collections::HashMap;
use tera::{Value, to_value, Result};
use std::panic;
use xss::*;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct IdContext {
    pub user_name: String,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub updated_at: NaiveDateTime,
    pub tags: Vec<Tag>
}

#[derive(Serialize)]
pub struct BookContext {
    pub user_id: i32,
    pub user_name: String
}

pub fn filter_markdown (value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("markdown", "value", String, value);
    let parser = Parser::new(&s);
    let mut buff = String::new();
    html::push_html(&mut buff, parser);
    Ok(to_value(buff).unwrap())
}

pub fn filter_description (value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let mut s = try_get_value!("description", "value", String, value);
    let description = s.chars().take(300).filter(|c| c != &'#' && c != &'\n' && c != &'\r').collect::<String>();
    Ok(to_value(format!("<meta name=\"description\" content=\"{}\">", description)).unwrap())
}

pub fn filter_capitalize (value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("capitalize", "value", String, value);
    let mut chars = s.chars();
    let title = match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str()
    };
    Ok(to_value(title).unwrap())
}

pub fn filter_title (value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("title", "value", String, value);
    let mut chars = s.chars();
    let title = match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str()
    };
    Ok(to_value(format!("<a href=\"/{}\" class=\"title-global js\">{}'s</a>", s, title)).unwrap())
}

pub fn filter_decode (value: Value, _: HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("decode", "value", String, value);
    let decoded = htmlescape::decode_html(&s).unwrap();
    Ok(to_value(decoded).unwrap())
}

pub fn filter_sanitize (value: Value, _:HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("sanitize", "value", String, value);
    let mut xss = xss::new();
    xss.set_allow_tag("a", vec!["href", "target", "style", "class"]);
    xss.set_allow_tag("p", vec!["style", "class"]);
    xss.set_allow_tag("h1", vec!["style", "class"]);
    xss.set_allow_tag("h2", vec!["style", "class"]);
    xss.set_allow_tag("h3", vec!["style", "class"]);
    xss.set_allow_tag("h4", vec!["style", "class"]);
    xss.set_allow_tag("div", vec!["style", "class"]);
    xss.set_allow_tag("table", vec!["style", "class"]);
    xss.set_allow_tag("tr", vec!["style", "class"]);
    xss.set_allow_tag("td", vec!["style", "class"]);
    xss.set_allow_tag("th", vec!["style", "class"]);
    xss.set_allow_tag("pre", vec!["style", "class"]);
    xss.set_allow_tag("span", vec!["style", "class"]);
    xss.set_allow_tag("ul", vec!["style", "class"]);
    xss.set_allow_tag("li", vec!["style", "class"]);
    xss.set_allow_tag("ol", vec!["style", "class"]);
    xss.set_allow_tag("code", vec!["style", "class"]);
    xss.set_allow_tag("em", vec!["style", "class"]);
    xss.set_allow_tag("blockquote", vec!["style", "class"]);
    xss.set_allow_tag("hr", vec!["style", "class"]);
    xss.set_allow_tag("br", vec!["style", "class"]);
    xss.set_allow_tag("cite", vec!["style", "class"]);
    let sanitized = xss.sanitize(&s);
    Ok(to_value(sanitized).unwrap())
}