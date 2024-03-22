use rb_sys::{
    rb_define_module, rb_define_singleton_method, rb_str_buf_append,
    rb_utf8_str_new_cstr, VALUE,
};
use std::{intrinsics::transmute, os::raw::c_char};

// Converts a static &str to a C string usable in foreign functions.
macro_rules! static_cstring {
    ($string:expr) => {{
        concat!($string, "\0").as_ptr() as *const c_char
    }};
}

fn inner_hello(name: VALUE) -> VALUE {
    let ret: VALUE;
    unsafe {
        ret = rb_str_buf_append(
        rb_utf8_str_new_cstr(static_cstring!("Hello, ")), name
    );
    }
    return ret;
}

unsafe extern "C" fn hello(_: VALUE, name: VALUE) -> VALUE {
    return inner_hello(name);
}

#[no_mangle]
unsafe extern "C" fn Init_another_parser() {
    let module = rb_define_module(static_cstring!("AnotherParser"));

    rb_define_singleton_method(
        module,
        static_cstring!("hello"),
        Some(transmute::<unsafe extern "C" fn(VALUE, VALUE) -> VALUE, _>(
            hello,
        )),
        1,
    );
}