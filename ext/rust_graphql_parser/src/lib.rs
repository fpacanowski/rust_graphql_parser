extern crate graphql_parser;

mod translation;

use graphql_parser::query::parse_query;

use magnus::{define_module, exception, function, prelude::*, Error, RHash};

fn hello(subject: String) -> String {
    format!("Hello from Rust, {}!", subject)
}

type TextType = String;

fn parse(query: String) -> Result<RHash, Error> {
    let res = std::hint::black_box(parse_query::<TextType>(&query));
    std::hint::black_box(format!("#{:?}",res));
    match res {
        Ok(r) => return Ok(translation::translate_document(&r)),
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
