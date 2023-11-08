extern crate graphql_parser;

use graphql_parser::query::{parse_query, ParseError, Document};

use magnus::{define_module, exception, function, prelude::*, Error, RHash};

fn hello(subject: String) -> String {
    format!("Hello from Rust, {}!", subject)
}

fn parse(query: String) -> Result<RHash, Error> {
    let hash = RHash::new();
    hash.aset("foo", 7).unwrap();
    let res = graphql_parser::query::parse_query::<&str>(&query);
    match res {
        Ok(r) => return Ok(hash),
        Err(e) => return Err(Error::new(exception::runtime_error(), e.to_string())),
    }
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustGraphqlParser")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    module.define_singleton_method("parse", function!(parse, 1))?;
    Ok(())
}
