#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use rocket::routes;
use mlsql_app_store::controller;
use mlsql_app_store::base::db::MyPool;
use mlsql_app_store::base::config::Config;

fn main() {
    let config = Config::new();
    let db = MyPool::new(&config);
    rocket::ignite().manage(db).mount("/", routes![controller::store_controller::showTable]).launch();
}