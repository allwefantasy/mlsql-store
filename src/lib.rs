#![feature(proc_macro_hygiene, decl_macro, async_await)]
#![feature(fn_traits)]
#![feature(async_closure)]

extern crate clap;
extern crate reqwest;
extern crate rocket;
extern crate serde;
extern crate vlog;
extern crate rustc_serialize;

pub mod controller;
pub mod base;
pub mod model;