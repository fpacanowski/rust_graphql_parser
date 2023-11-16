extern crate graphql_parser;

mod translation;

use graphql_parser::query::parse_query;

use magnus::{define_module, exception, function, prelude::*, Error, RHash};

fn parse(query: String) -> Result<RHash, Error> {
    match parse_query::<String>(&query) {
        Ok(r) => return Ok(translation::translate_document(&r)),
        Err(e) => return Err(Error::new(exception::runtime_error(), e.to_string())),
    }
}

fn parse_raw(query: String) -> String {
    let ast = parse_query::<String>(&query);
    return format!("#{:?}",ast);
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("RustGraphqlParser")?;
    module.define_singleton_method("parse", function!(parse, 1))?;
    module.define_singleton_method("parse_raw", function!(parse_raw, 1))?;
    Ok(())
}
