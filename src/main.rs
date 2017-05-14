
extern crate dynparser;
use dynparser::grammar::grammar;


use dynparser::{symbol, text2parse, parse};


fn main() {
    let parsed = parse(&text2parse(r#"id=[_] [_]"#), &symbol("grammar"), &grammar());

    match parsed {
        Err(err) => println!("error... {} ___________", err),
        Ok(res) => println!("Ok... {:?} ___________", res),
    };
}
