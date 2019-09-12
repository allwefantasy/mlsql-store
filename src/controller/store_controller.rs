use crate::base::db::MyPool;
use crate::model::models::PluginsStore;
use mysql as my;
use rocket::http::Status;
use rocket::request::{Form, FromForm};
use rocket::response::{content, NamedFile, Responder, Stream};
use rocket::{get, post, Request, Response, State};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

#[get("/jack")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/repo/plugins?<name>")]
pub fn plugin_store_list(db: State<MyPool>, name: String) -> content::Json<String> {
    let db_ref = db.value();
    let sql = "select id,name,location,official,enable from plugins_store where enable=?";
    let res = PluginsStore::query(&db, sql, (0,));
    match res {
        Ok(item) => content::Json(serde_json::to_string(&item).unwrap()),
        Err(e) => content::Json(String::from("[]")),
    }
}

#[post("/repo/plugins/download", data = "<req>")]
pub fn download_plugin(db: State<MyPool>, req: Form<DownloadReq>) -> PathResp {
    let db_ref = db.value();
    let name = &req.name.clone();
    let res: Vec<PluginsStore> = PluginsStore::query(
        &db,
        "select id,name,location,official,enable from plugins_store where name=?",
        (&req.name,),
    )
    .unwrap();
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

pub enum PathResp {
    File(PathBuf),
    ErrorMessage(String),
}

impl Responder<'static> for PathResp {
    fn respond_to(self, req: &Request) -> Result<Response<'static>, Status> {
        match self {
            PathResp::File(path) => {
                let cd = format!(
                    "attachment;filename=\"{}\"",
                    &path.file_name().unwrap().to_str().unwrap()
                );
                Response::build_from(NamedFile::open(&path).ok().respond_to(req)?)
                    .raw_header("ContentType", "application/octet-stream")
                    .raw_header("Content-Disposition", cd)
                    .ok()
            }
            PathResp::ErrorMessage(msg) => content::Plain(msg).respond_to(req),
        }
    }
}
