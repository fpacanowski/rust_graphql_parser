use graphql_parser::query::{
    Definition, Document, Field, FragmentDefinition, FragmentSpread, InlineFragment, Mutation,
    OperationDefinition, Query, Selection, SelectionSet, Subscription, TypeCondition,
    VariableDefinition,
};
use graphql_parser::schema::{Directive, Type, Value};
use graphql_parser::Pos;
use rb_sys::{VALUE, rb_intern, rb_hash_new, rb_id2sym, rb_hash_aset, rb_hash_bulk_insert, rb_ary_new_capa, rb_ary_push};


macro_rules! static_cstring {
    ($string:expr) => {{
        concat!($string, "\0").as_ptr() as *const std::os::raw::c_char
    }};
}
unsafe fn resolve(class_name: *const std::os::raw::c_char) -> VALUE {
    let cGraphQL = rb_sys::rb_const_get(
        rb_sys::rb_cObject, 
        rb_intern(static_cstring!("GraphQL")));
    let cLanguage = rb_sys::rb_const_get(
        cGraphQL, 
        rb_intern(static_cstring!("Language")));
    let cNodes = rb_sys::rb_const_get(
        cLanguage, 
        rb_intern(static_cstring!("Nodes")));
    
    return rb_sys::rb_const_get(
        cNodes,
        rb_intern(class_name));

}
pub unsafe fn translate_document(doc: &Document<'_, String>) -> VALUE {
    // GraphQL::Language::Nodes
    let cGraphQL = rb_sys::rb_const_get(
        rb_sys::rb_cObject, 
        rb_intern(static_cstring!("GraphQL")));
    let cLanguage = rb_sys::rb_const_get(
        cGraphQL, 
        rb_intern(static_cstring!("Language")));
    let cNodes = rb_sys::rb_const_get(
        cLanguage, 
        rb_intern(static_cstring!("Nodes")));
    
    let class = rb_sys::rb_const_get(
        cNodes,
        rb_intern(static_cstring!("Document")));
    // let hash = build_ruby_node("document");
    // let definitions = RArray::new();
    // for x in doc.definitions.iter() {
    //     definitions.push(translate_definition(x)).unwrap();
    // }
    // hash.aset(Symbol::new("definitions"), definitions).unwrap();
    // return rb_sys::rb_int2inum(7);
    let definitions = rb_sys::rb_ary_new();
    for x in doc.definitions.iter() {
        rb_sys::rb_ary_push(definitions, translate_definition(x));
    }
    let kwargs = rb_hash_new();
    rb_sys::rb_hash_aset(
        kwargs, rb_sys::rb_id2sym(rb_intern(static_cstring!("definitions"))), definitions);
    return rb_sys::rb_class_new_instance_kw(1, &kwargs, class, 1);
    // return rb_sys::rb_class_new_instance(0, std::ptr::null(), class);
    // return rb_sys::Qnil.into();
}

unsafe fn translate_definition(definition: &Definition<'_, String>) -> VALUE {
    return match definition {
        Definition::Operation(operation) => rb_sys::Qnil.into(),
        Definition::Fragment(fragment) => translate_fragment_definition(fragment),
    };
}

unsafe fn ruby_str(rust_str: &str) -> VALUE {
    rb_sys::rb_str_new(rust_str.as_ptr() as _,rust_str.len() as _)
}

unsafe fn translate_fragment_definition(fragment_definition: &FragmentDefinition<'_, String>) -> VALUE {
    let kwargs = build_hash(&[
        *symbols::NAME, ruby_str(&fragment_definition.name),
        *symbols::TYPE, translate_type_condition(&fragment_definition.type_condition),
        *symbols::SELECTIONS, translate_selection_set(&fragment_definition.selection_set),
    ]);

    let class = resolve(static_cstring!("FragmentDefinition"));
    return rb_sys::rb_class_new_instance_kw(1, &kwargs, class, 1);
    // hash.aset(
    //     Symbol::new("selection_set"),
    //     translate_selection_set(&fragment_definition.selection_set),
    // )
    // .unwrap();
}

unsafe fn translate_type_condition(type_condition: &TypeCondition<'_, String>) -> VALUE {
    let TypeCondition::On(type_name) = type_condition;
    let kwargs = build_hash(&[*symbols::NAME, ruby_str(type_name)]);
    let class = resolve(static_cstring!("TypeName"));
    return rb_sys::rb_class_new_instance_kw(1, &kwargs, class, 1);
}

unsafe fn translate_selection_set(selection_set: &SelectionSet<'_, String>) -> VALUE {
    let result: VALUE = rb_ary_new_capa(selection_set.items.len() as _);
    for x in selection_set.items.iter() {
        rb_ary_push(result, translate_selection(x));
    }
    return result;
    // let hash = RHash::new();
    // hash.aset(Symbol::new("node_type"), Symbol::new("selection_set"))
    //     .unwrap();

    // let span = RArray::new();
    // span.push(translate_position(&selection_set.span.0))
    //     .unwrap();
    // span.push(translate_position(&selection_set.span.1))
    //     .unwrap();
    // hash.aset(Symbol::new("span"), span).unwrap();

    // let items = RArray::new();
    // for x in selection_set.items.iter() {
    //     items.push(translate_selection(x)).unwrap();
    // }
    // hash.aset(Symbol::new("items"), items).unwrap();

    // return hash;
}

unsafe fn translate_selection(selection: &Selection<'_, String>) -> VALUE {
    return match selection {
        Selection::Field(field) => translate_field(field),
        Selection::FragmentSpread(fragment_spread) => translate_fragment_spread(fragment_spread),
        Selection::InlineFragment(inline_fragment) => translate_inline_fragment(inline_fragment),
    };
}

unsafe fn translate_field(field: &Field<'_, String>) -> VALUE {
    let kwargs = build_hash(&[*symbols::NAME, ruby_str(&field.name)]);
    let class = resolve(static_cstring!("Field"));
    return rb_sys::rb_class_new_instance_kw(1, &kwargs, class, 1);
}

unsafe fn translate_directive(directive: &Directive<'_, String>) -> VALUE {
    unimplemented()
}

unsafe fn translate_fragment_spread(fragment_spread: &FragmentSpread<'_, String>) -> VALUE {
    unimplemented()
}

unsafe fn translate_inline_fragment(inline_fragment: &InlineFragment<'_, String>) -> VALUE {
    unimplemented()
}

mod symbols {
    use rb_sys::{VALUE, rb_intern, rb_hash_new, rb_id2sym, rb_hash_aset, rb_hash_bulk_insert};
    use once_cell::sync::Lazy;
    pub static NAME: Lazy<VALUE> = Lazy::new(|| unsafe {
        rb_id2sym(rb_intern!("name"))
    });
    pub static TYPE: Lazy<VALUE> = Lazy::new(|| unsafe {
        rb_id2sym(rb_intern!("type"))
    });
    pub static SELECTIONS: Lazy<VALUE> = Lazy::new(|| unsafe {
        rb_id2sym(rb_intern!("selections"))
    });
}

unsafe fn unimplemented() -> VALUE {
    // let result = rb_hash_new();
    // rb_hash_aset(
    //     result,
    //     rb_id2sym(rb_intern(static_cstring!("unimplemented"))),
    //     rb_sys::Qtrue as _
    // );
    // return result;
    return build_hash(&[rb_id2sym(rb_intern(static_cstring!("unimplemented"))), rb_sys::Qtrue as _])
}

unsafe fn build_hash(arr: &[VALUE]) -> VALUE {
    let result = rb_hash_new();
    rb_hash_bulk_insert(arr.len() as _, arr.as_ptr(), result);
    return result;
}