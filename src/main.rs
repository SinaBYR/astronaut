use parsers::{
    html,
    css,
};

pub mod parsers;

fn main() {
    let root = parsers::html::parse(String::from("<div><p id=\"hello\">Hello world</p></div>"));
    let rules = parsers::css::parse(String::from("#hello { display: block; width: 250px; }")).rules;
}

