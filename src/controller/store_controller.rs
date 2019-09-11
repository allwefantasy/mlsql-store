use rocket::response::content;
use rocket::{get, State};
use serde::{Deserialize, Serialize};
use crate::base::db::MyPool;
use mysql as my;
use std::sync::mpsc::Sender;

#[get("/jack")]
pub fn showTable(db: State<MyPool>) -> content::Json<String> {
    let db_ref = db.value();
    let query = async move |tx_c: Sender<Result<Vec<HelloRes>, my::error::Error>>| {
        let query_res = db_ref.prep_exec(
            "select name,url from backend", ()).unwrap();

        let res = query_res.map(|x| x.unwrap()).map(|row| {
            let (name, url) = my::from_row::<(String, String)>(row);
            HelloRes { name, url }
        }).collect::<Vec<HelloRes>>();
        tx_c.send(Ok(res)).unwrap();
        ()
    };
    let res = db.future_exec(query);
    match res {
        Ok(item) => content::Json(serde_json::to_string(&item).unwrap()),
        Err(e) => content::Json(String::from("{}")),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HelloRes {
    name: String,
    url: String,
}