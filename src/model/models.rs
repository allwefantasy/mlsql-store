use mysql::Params;
use mysql as my;
use serde_json;
use serde::{Deserialize, Serialize};
use std::sync::mpsc::Sender;
use crate::base::db::MyPool;
use std::borrow::BorrowMut;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginsStore {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub official: i32,
    pub enable: i32,
    pub author: String,
    pub version: String,
    pub mlsql_versions: String,
    pub created_time: i64,
    pub github_url: String,
    pub plugin_type: i32,
    pub desc: String,
    pub main_class: String,
}

pub enum PluginType {
    ET = 0,
    DataSource = 1,
    SCRIPT = 2,
    APP = 3,
}

impl PluginType {
    pub fn fromValue(v: i32) -> PluginType {
        match v {
            0 => PluginType::ET,
            1 => PluginType::DataSource,
            2 => PluginType::SCRIPT,
            3 => PluginType::APP,
            _ => PluginType::ET
        }
    }
}

impl PluginsStore {
    //where T:Into<Params>
    pub fn query<T>(db: &MyPool, sql: &str, params: T) -> Result<Vec<PluginsStore>, my::error::Error> where T: Into<Params> {
        let db_ref = db.value();
        let query = async move |tx_c: Sender<Result<Vec<PluginsStore>, my::error::Error>>| {
            let query_res = db_ref.prep_exec(
                sql, params).unwrap();

            let res = query_res.map(|x| x.unwrap()).map(|mut row| {
                PluginsStore {
                    id: row.take("id").unwrap(),
                    name: row.take("name").unwrap(),
                    location: row.take("location").unwrap(),
                    official: row.take("official").unwrap(),
                    enable: row.take("enable").unwrap(),
                    author: row.take("author").unwrap(),
                    version: row.take("version").unwrap(),
                    mlsql_versions: row.take("mlsql_versions").unwrap(),
                    created_time: row.take("created_time").unwrap(),
                    github_url: row.take("github_url").unwrap(),
                    plugin_type: row.take("plugin_type").unwrap(),
                    desc: row.take("desc").unwrap(),
                    main_class: row.take("main_class").unwrap(),
                }
            }).collect::<Vec<PluginsStore>>();
            tx_c.send(Ok(res)).unwrap();
            ()
        };
        db.future_exec(query)
    }
}