// This includes openGL C bindings made available to us through the build Process,
// more specifically the amazing "bindgen" crate
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));