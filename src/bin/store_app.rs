#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use mlsql_app_store::base::config::Config;
use mlsql_app_store::base::db::MyPool;
use mlsql_app_store::controller;
use rocket::config::Environment;
use rocket::routes;
use rocket_contrib::serve::StaticFiles;

fn main() {
    let config = Config::new();
    let db = MyPool::new(&config);

    let server_config = rocket::Config::build(Environment::Staging)
        .address("localhost")
        .port(8001)
        .finalize()
        .unwrap();
    rocket::custom(server_config)
        .manage(db)
        .mount(
            "/",
            routes![
                controller::store_controller::index,
                controller::store_controller::download_plugin,
                controller::store_controller::plugin_store_list
            ],
        )
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
