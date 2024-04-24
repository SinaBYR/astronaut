use std::{char, collections::HashMap};

use parsers::html;

pub mod parsers;

fn main() {
    // <div>
    //     <p>hello</p>
    //     world
    // </div>
    let root = parsers::html::parse(String::from("<html><p id=\"123\">Hello world</p></html>"));
}
