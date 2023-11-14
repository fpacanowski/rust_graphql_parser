use graphql_parser::query::{
    Definition, Document, Field, FragmentDefinition, FragmentSpread, InlineFragment,
    OperationDefinition, Query, Selection, SelectionSet, TypeCondition, VariableDefinition, Mutation,
};
use graphql_parser::schema::{Type, Value, Directive};
use graphql_parser::Pos;

use magnus::{RArray, RHash, Symbol};

type TextType = String;

pub fn translate_document(doc: &Document<'_, TextType>) -> RHash {
    // println!("{:#?}", doc);
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
        Definition::Fragment(fragment) => translate_fragment_definition(fragment),
    };
}

fn translate_operation_definition(
    operation_definition: &OperationDefinition<'_, TextType>,
) -> RHash {
    return match operation_definition {
        OperationDefinition::Query(query) => translate_query(query),
        OperationDefinition::SelectionSet(selection_set) => translate_selection_set(selection_set),
        OperationDefinition::Mutation(mutation) => translate_mutation(mutation),
        OperationDefinition::Subscription(_subscription) => unimplemented(),
    };
}

fn translate_fragment_definition(fragment_definition: &FragmentDefinition<'_, TextType>) -> RHash {
    let hash = build_ruby_node("fragment_definition");
    hash.aset(Symbol::new("name"), fragment_definition.name.clone())
        .unwrap();
    hash.aset(
        Symbol::new("position"),
        translate_position(&fragment_definition.position),
    )
    .unwrap();
    hash.aset(
        Symbol::new("selection_set"),
        translate_selection_set(&fragment_definition.selection_set),
    )
    .unwrap();

    let type_condition = RHash::new();
    let TypeCondition::On(on_type) = &fragment_definition.type_condition;
    type_condition
        .aset(Symbol::new("on"), on_type.clone())
        .unwrap();
    hash.aset(Symbol::new("type_condition"), type_condition)
        .unwrap();
    return hash;
}

fn translate_query(query: &Query<'_, TextType>) -> RHash {
    let hash = RHash::new();
    hash.aset(Symbol::new("node_type"), Symbol::new("query"))
        .unwrap();
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

    let variable_definitions = RArray::new();
    for x in query.variable_definitions.iter() {
        variable_definitions
            .push(translate_variable_definition(x))
            .unwrap();
    }
    hash.aset(Symbol::new("variable_definitions"), variable_definitions)
        .unwrap();

    let directives = RArray::new();
    for directive in query.directives.iter() {
        directives.push(translate_directive(directive)).unwrap();
    }
    hash.aset(Symbol::new("directives"), directives).unwrap();

    return hash;
}

// TODO: unify with translate_query.
fn translate_mutation(query: &Mutation<'_, TextType>) -> RHash {
    let hash = RHash::new();
    hash.aset(Symbol::new("node_type"), Symbol::new("mutation"))
        .unwrap();
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

    let variable_definitions = RArray::new();
    for x in query.variable_definitions.iter() {
        variable_definitions
            .push(translate_variable_definition(x))
            .unwrap();
    }
    hash.aset(Symbol::new("variable_definitions"), variable_definitions)
        .unwrap();

    let directives = RArray::new();
    for directive in query.directives.iter() {
        directives.push(translate_directive(directive)).unwrap();
    }
    hash.aset(Symbol::new("directives"), directives).unwrap();

    return hash;
}

fn translate_variable_definition(variable_definition: &VariableDefinition<'_, String>) -> RHash {
    let hash = build_ruby_node("variable_definition");
    hash.aset(Symbol::new("name"), variable_definition.name.clone())
        .unwrap();
    hash.aset(
        Symbol::new("position"),
        translate_position(&variable_definition.position),
    )
    .unwrap();
    hash.aset(
        Symbol::new("var_type"),
        translate_type(&variable_definition.var_type),
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
        Selection::FragmentSpread(fragment_spread) => translate_fragment_spread(fragment_spread),
        Selection::InlineFragment(inline_fragment) => translate_inline_fragment(inline_fragment),
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

    let arguments = RArray::new();
    for (name, val) in field.arguments.iter() {
        arguments.push(translate_argument(name, val)).unwrap();
    }
    hash.aset(Symbol::new("arguments"), arguments).unwrap();

    let directives = RArray::new();
    for directive in field.directives.iter() {
        directives.push(translate_directive(directive)).unwrap();
    }
    hash.aset(Symbol::new("directives"), directives).unwrap();

    return hash;
}

fn translate_directive(directive: &Directive<'_, TextType>) -> RHash {
    let hash = build_ruby_node("directive");
    hash.aset(Symbol::new("name"), directive.name.clone()).unwrap();
    hash.aset(Symbol::new("position"), translate_position(&directive.position))
        .unwrap();

    let arguments = RArray::new();
    for (name, val) in directive.arguments.iter() {
        arguments.push(translate_argument(name, val)).unwrap();
    }
    hash.aset(Symbol::new("arguments"), arguments).unwrap();

    return hash;
}

fn translate_fragment_spread(fragment_spread: &FragmentSpread<'_, TextType>) -> RHash {
    let hash = build_ruby_node("fragment_spread");
    hash.aset(
        Symbol::new("fragment_name"),
        fragment_spread.fragment_name.clone(),
    )
    .unwrap();
    hash.aset(
        Symbol::new("position"),
        translate_position(&fragment_spread.position),
    )
    .unwrap();
    return hash;
}

fn translate_inline_fragment(inline_fragment: &InlineFragment<'_, TextType>) -> RHash {
    let hash = build_ruby_node("inline_fragment");
    hash.aset(
        Symbol::new("position"),
        translate_position(&inline_fragment.position),
    )
    .unwrap();
    hash.aset(
        Symbol::new("selection_set"),
        translate_selection_set(&inline_fragment.selection_set),
    )
    .unwrap();
    if let Some(TypeCondition::On(on_type)) = &inline_fragment.type_condition {
        let type_condition = RHash::new();
        type_condition
            .aset(Symbol::new("on"), on_type.clone())
            .unwrap();
        hash.aset(Symbol::new("type_condition"), type_condition)
            .unwrap();
    }
    let directives = RArray::new();
    for directive in inline_fragment.directives.iter() {
        directives.push(translate_directive(directive)).unwrap();
    }
    hash.aset(Symbol::new("directives"), directives).unwrap();
    return hash;
}

fn translate_type(type_def: &Type<'_, TextType>) -> RHash {
    return match type_def {
        Type::NamedType(type_name) => {
            let hash = build_ruby_node("named_type");
            hash.aset(Symbol::new("name"), type_name.clone()).unwrap();
            hash
        }
        Type::ListType(inner_type) => {
            let hash = build_ruby_node("list_type");
            hash.aset(Symbol::new("type"), translate_type(inner_type)).unwrap();
            hash
        },
        Type::NonNullType(inner_type) => {
            let hash = build_ruby_node("non_null_type");
            hash.aset(Symbol::new("type"), translate_type(inner_type)).unwrap();
            hash
        },
    };
}

fn translate_argument(name: &String, val: &Value<'_, TextType>) -> RHash {
    let hash = build_ruby_node("argument");
    hash.aset(Symbol::new("name"), name.clone()).unwrap();
    hash.aset(Symbol::new("value"), translate_value(val)).unwrap();
    return hash;
}

fn translate_value(value: &Value<'_, TextType>) -> RHash {
    return match value {
        Value::Variable(variable) => {
            let res = build_ruby_node("variable");
            res.aset(Symbol::new("name"), variable.clone()).unwrap();
            return res;
        },
        Value::Int(number) => {
            let res = build_ruby_node("int");
            res.aset(Symbol::new("value"), number.as_i64()).unwrap();
            return res;
        },
        Value::Float(number) => {
            let res = build_ruby_node("float");
            res.aset(Symbol::new("value"), *number).unwrap();
            return res;
        },
        Value::String(str) => {
            let res = build_ruby_node("string");
            res.aset(Symbol::new("value"), str.clone()).unwrap();
            return res;
        },
        Value::Boolean(bool) => {
            let res = build_ruby_node("boolean");
            res.aset(Symbol::new("value"), *bool).unwrap();
            return res;
        },
        Value::Null => build_ruby_node("null"),
        Value::Enum(enum_name) => {
            let res = build_ruby_node("enum");
            res.aset(Symbol::new("value"), enum_name.clone()).unwrap();
            return res;
        },
        Value::List(vals) => {
            let res = build_ruby_node("list");
            let value = RArray::new();
            for v in vals.iter() {
                value.push(translate_value(v)).unwrap();
            }
            res.aset(Symbol::new("value"), value).unwrap();
            return res;
        },
        Value::Object(obj) => {
            let res = build_ruby_node("object");
            let value = RArray::new();
            for (name, val) in obj.iter() {
                value.push(translate_argument(name, val)).unwrap();
            }
            res.aset(Symbol::new("value"), value).unwrap();
            return res;
        },
    }
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
    return build_ruby_node("unimplemented");
}
