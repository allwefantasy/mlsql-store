use mysql::Params;
use mysql as my;
use serde_json;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use crate::base::db::MyPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginsStore {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub official: i32,
    pub enable: i32,
}

impl PluginsStore {
    //where T:Into<Params>
    pub fn query<T>(db: &MyPool, sql: &str, params: T) -> Result<Vec<PluginsStore>, my::error::Error> where T: Into<Params> {
        let db_ref = db.value();
        let query = async move |tx_c: Sender<Result<Vec<PluginsStore>, my::error::Error>>| {
            let query_res = db_ref.prep_exec(
                sql, params).unwrap();

            let res = query_res.map(|x| x.unwrap()).map(|row| {
                let (id, name, location, official, enable) = my::from_row::<(i32, String, String, i32, i32)>(row);
                PluginsStore { id, name, location, official, enable }
            }).collect::<Vec<PluginsStore>>();
            tx_c.send(Ok(res)).unwrap();
            ()
        };
        db.future_exec(query)
    }
}