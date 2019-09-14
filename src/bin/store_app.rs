#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate clap;

use clap::{App, AppSettings, Arg, crate_authors, crate_version};

use rocket::routes;
use mlsql_app_store::controller;
use mlsql_app_store::base::db::MyPool;
use mlsql_app_store::base::config::Config;
use rocket::config::Environment;
use rocket_contrib::serve::StaticFiles;

fn main() {
    let app_params = App::new("MLSQL Store")
        .about("MLSQL Store Website")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("h")
                .long("host")
                .takes_value(true)
                .help("bind host").default_value("localhost")
        ).arg(
        Arg::with_name("p")
            .long("port")
            .takes_value(true)
            .help("bind port").default_value("8001")
    );
    let matches = app_params.clone().get_matches();

    let host = matches.value_of("h").unwrap();
    let port = matches.value_of("p").unwrap();

    let config = Config::new();
    let db = MyPool::new(&config);

    let server_config = rocket::Config::build(Environment::Staging)
        .address(host)
        .port(port.parse::<u16>().unwrap())
        .finalize().unwrap();
    rocket::custom(server_config).manage(db).mount("/", routes![
    controller::store_controller::index,
    controller::store_controller::download_plugin,
    controller::store_controller::plugin_list
    ]).mount("/public", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))).launch();
}