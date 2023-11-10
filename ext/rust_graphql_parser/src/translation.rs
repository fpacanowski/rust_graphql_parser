use graphql_parser::query::{
    Definition, Document, Field, OperationDefinition, Query, Selection, SelectionSet,
};

use graphql_parser::Pos;

use magnus::{RArray, RHash, Symbol};

type TextType = String;

pub fn translate_document(doc: &Document<'_, TextType>) -> RHash {
    let hash = build_ruby_node("document");
    let definitions = RArray::new();
    for x in doc.definitions.iter() {
        definitions.push(translate_definition(x)).unwrap();
    }
    hash.aset(Symbol::new("definitions"), definitions).unwrap();
    return hash;
}

fn translate_definition(definition: &Definition<'_, TextType>) -> RHash {
    return match definition {
        Definition::Operation(operation) => translate_operation_definition(operation),
        Definition::Fragment(_fragment) => unimplemented(),
    };
}

fn translate_operation_definition(
    operation_definition: &OperationDefinition<'_, TextType>,
) -> RHash {
    return match operation_definition {
        OperationDefinition::Query(query) => translate_query(query),
        OperationDefinition::SelectionSet(_selection_set) => unimplemented(),
        OperationDefinition::Mutation(_mutation) => unimplemented(),
        OperationDefinition::Subscription(_subscription) => unimplemented(),
    };
}

fn translate_query(query: &Query<'_, TextType>) -> RHash {
    let hash = RHash::new();
    hash.aset(Symbol::new("node_type"), Symbol::new("query"))
        .unwrap();
    // println!("dupa: {:#?}", query);
    if let Some(query_name) = query.name.clone() {
        hash.aset(Symbol::new("name"), query_name.clone()).unwrap();
    }
    hash.aset(Symbol::new("position"), translate_position(&query.position))
        .unwrap();
    hash.aset(
        Symbol::new("selection_set"),
        translate_selection_set(&query.selection_set),
    )
    .unwrap();
    return hash;
}

fn translate_selection_set(selection_set: &SelectionSet<'_, TextType>) -> RHash {
    let hash = RHash::new();
    hash.aset(Symbol::new("node_type"), Symbol::new("selection_set"))
        .unwrap();

    let span = RArray::new();
    span.push(translate_position(&selection_set.span.0))
        .unwrap();
    span.push(translate_position(&selection_set.span.1))
        .unwrap();
    hash.aset(Symbol::new("span"), span).unwrap();

    let items = RArray::new();
    for x in selection_set.items.iter() {
        items.push(translate_selection(x)).unwrap();
    }
    hash.aset(Symbol::new("items"), items).unwrap();

    return hash;
}

fn translate_selection(selection: &Selection<'_, TextType>) -> RHash {
    return match selection {
        Selection::Field(field) => translate_field(field),
        Selection::FragmentSpread(_fragment_spread) => unimplemented(),
        Selection::InlineFragment(_inline_fragment) => unimplemented(),
    };
}

fn translate_field(field: &Field<'_, TextType>) -> RHash {
    let hash = build_ruby_node("field");
    hash.aset(Symbol::new("name"), field.name.clone()).unwrap();
    hash.aset(Symbol::new("position"), translate_position(&field.position))
        .unwrap();
    hash.aset(
        Symbol::new("selection_set"),
        translate_selection_set(&field.selection_set),
    )
    .unwrap();
    return hash;
}

fn translate_position(position: &Pos) -> RHash {
    let hash = RHash::new();
    hash.aset(Symbol::new("line"), position.line).unwrap();
    hash.aset(Symbol::new("column"), position.column).unwrap();
    return hash;
}

fn build_ruby_node(node_type: &str) -> RHash {
    let hash = RHash::new();
    hash.aset(Symbol::new("node_type"), Symbol::new(node_type))
        .unwrap();
    return hash;
}

fn unimplemented() -> RHash {
    let hash = RHash::new();
    hash.aset(Symbol::new("unimplemented"), true).unwrap();
    return hash;
}
