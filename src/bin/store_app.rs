#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use rocket::routes;
use mlsql_app_store::controller;
fn main() {
    rocket::ignite().mount("/", routes![controller::store_controller::get_hello]).launch();
}