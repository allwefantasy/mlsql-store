use rocket::response::{content, Stream, NamedFile, Responder};
use rocket::{post, get, State, Request, Response};
use rocket::request::{FromForm, Form};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use mysql as my;
use std::sync::mpsc::Sender;
use std::io;
use std::fs::File;
use serde_json;
use std::path::PathBuf;
use crate::base::db::MyPool;
use crate::model::models::PluginsStore;


#[get("/jack")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/repo/plugins?<plugin_type>")]
pub fn plugin_list(db: State<MyPool>, plugin_type: String) -> content::Json<String> {
    let db_ref = db.value();
    let sql = "select * from plugins_store where enable=? and plugin_type=?";
    let res = PluginsStore::query(&db, sql, (0, plugin_type.parse::<i32>().unwrap()));
    match res {
        Ok(item) => content::Json(serde_json::to_string(&item).unwrap()),
        Err(e) => content::Json(String::from("[]")),
    }
}


#[post("/repo/plugins/download", data = "<req>")]
pub fn download_plugin(db: State<MyPool>, req: Form<DownloadReq>) -> PathResp {
    let db_ref = db.value();
    let name = &req.name.clone();
    let res: Vec<PluginsStore> = PluginsStore::query(&db, "select * from plugins_store where name=?", (&req.name, )).unwrap();
    let zero: usize = 0;
    if &res.len() > &zero {
        PathResp::File(PathBuf::from(&res[0].location))
    } else {
        PathResp::ErrorMessage(String::from(format!("{} is not exists", name)))
    }
}


#[derive(FromForm)]
pub struct DownloadReq {
    name: String,
}

pub enum PathResp { File(PathBuf), ErrorMessage(String) }

impl Responder<'static> for PathResp {
    fn respond_to(self, req: &Request) -> Result<Response<'static>, Status> {
        match self {
            PathResp::File(path) => {
                let cd = format!("attachment;filename=\"{}\"", &path.file_name().unwrap().to_str().unwrap());
                Response::build_from(NamedFile::open(&path).ok().respond_to(req)?)
                    .raw_header("ContentType", "application/octet-stream")
                    .raw_header("Content-Disposition", cd)
                    .ok()
            }
            PathResp::ErrorMessage(msg) => {
                content::Plain(msg).respond_to(req)
            }
        }
    }
}