#![feature(libc)]

extern crate libc;
use libc::{c_void, c_int, c_char, c_uint, c_uchar};
use std::ffi::{CString,};
use std::ptr;

#[repr(C)]
pub struct zval;

#[repr(C)]
pub struct _zend_arg_info;
#[repr(C)]
pub struct _zend_ini_entry;
#[repr(C)]
pub struct _zend_module_dep;

#[repr(C)]
pub struct zend_module_entry {
    size:       u16,
    zend_api:   u32,
    zend_debug: c_char,
    zts:        c_char,
    ini_entry:  *const _zend_ini_entry,
    deps:       *const _zend_module_dep,
    name:       *const c_char,

    functions: *const zend_function_entry,
    module_startup_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    module_shutdown_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    request_startup_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    request_shutdown_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    info_func: Option<extern "C" fn(*const zend_module_entry)>,
    version: *const c_char,
    globals_size: usize,
    globals_ptr: *const c_void,
    globals_ctor: Option<extern "C" fn(*const c_void)>,
    globals_dtor: Option<extern "C" fn(*const c_void)>,
    post_deactivate_func: Option<extern "C" fn(c_void) -> c_int>,
    module_started: c_int,
    _type: c_uchar,
    handle: *const c_void,
    module_number: c_int,
    build_id: *const c_char
}

#[repr(C)]
pub struct zend_function_entry {
    fname:    *const c_char,
    handler:  Option<extern "C" fn(c_int, *mut zval, *mut *mut zval, *mut zval, c_int)>,
    arg_info: *const _zend_arg_info,
    num_args: c_uint,
    flags:    c_uint
}

//extern {
    //fn zend_parse_parameters(num_args: c_int, type_spce: *const c_char, ...);
//}

#[no_mangle]
pub extern "C" fn zif_confirm_rust_a_star_compiled(_: c_int, _: *mut zval, _: *mut *mut zval, _: *mut zval, _: c_int) {
//pub extern "C" fn zif_confirm_rust_a_star_compiled(ht: c_int, return_value: *mut zval, return_value_pointer: *mut *mut zval, this_pointer: *mut zval, return_value_used: c_int) {
    println!("lol");
}

#[no_mangle]
pub extern "C" fn zm_generic_rust_a_star(_: c_int, _: c_int) -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn zm_info_rust_a_star(_: *const zend_module_entry) { }

#[no_mangle]
pub extern "C" fn get_module(_: c_void) -> *const zend_module_entry {
    let rust_a_star_functions : &[zend_function_entry] = &[zend_function_entry {
        fname:    CString::new("confirm_rust_a_star_compiled").unwrap().as_ptr(),
        handler:  Some(zif_confirm_rust_a_star_compiled),
        arg_info: ptr::null(),
        num_args: 0,
        flags:    0
    },
    zend_function_entry {
        fname:    ptr::null(),
        handler:  None,
        arg_info: ptr::null(),
        num_args: 0,
        flags:    0
    }];

    let rust_a_star_module_entry = zend_module_entry {
        size:       std::mem::size_of::<zend_module_entry>() as u16,
        zend_api:   20131226,
        zend_debug: 0,
        zts:        0,
        ini_entry:  0 as *const _zend_ini_entry,
        deps:       0 as *const _zend_module_dep,
        name:       CString::new("rust_a_star").unwrap().as_ptr(),
        functions:  &rust_a_star_functions[0],

        module_startup_func:   Some(zm_generic_rust_a_star),
        module_shutdown_func:  Some(zm_generic_rust_a_star),
        request_startup_func:  Some(zm_generic_rust_a_star),
        request_shutdown_func: Some(zm_generic_rust_a_star),
        info_func:             Some(zm_info_rust_a_star),

        version:              CString::new("0.1.0").unwrap().as_ptr(),
        globals_size:         0,
        globals_ptr:          ptr::null(),
        globals_ctor:         None,
        globals_dtor:         None,
        post_deactivate_func: None,
        module_started:       0,
        _type:                0,
        handle:               ptr::null(),
        module_number:        0,
        build_id:             CString::new("API20131226,NTS,debug").unwrap().as_ptr()
    };
    &rust_a_star_module_entry
}
