#![feature(associated_consts)]
#![feature(custom_derive)]
#![feature(plugin)]

#![plugin(heapsize_plugin)]

extern crate heapsize;
extern crate uuid;

pub mod ast;
pub mod backend;
pub mod gc_error;
pub mod macros;
pub mod types;
