use rocket::response::{content, Stream, NamedFile, Responder};
use rocket::{post, get, State, Request, Response};
use rocket::request::{FromForm, Form};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use crate::base::db::MyPool;
use mysql as my;
use std::sync::mpsc::Sender;
use std::io;
use std::fs::File;
use serde_json;
use std::path::PathBuf;


#[get("/jack")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/repo/plugins?<name>")]
pub fn plugin_store_list(db: State<MyPool>, name: String) -> content::Json<String> {
    let db_ref = db.value();
    let query = async move |tx_c: Sender<Result<Vec<PluginsStore>, my::error::Error>>| {
        let query_res = db_ref.prep_exec(
            "select id,name,location,official,enable from plugins_store where name=?", (name, )).unwrap();

        let res = query_res.map(|x| x.unwrap()).map(|row| {
            let (id, name, location, official, enable) = my::from_row::<(i32, String, String, i32, i32)>(row);
            PluginsStore { id, name, location, official, enable }
        }).collect::<Vec<PluginsStore>>();
        tx_c.send(Ok(res)).unwrap();
        ()
    };
    let res = db.future_exec(query);
    match res {
        Ok(item) => content::Json(serde_json::to_string(&item).unwrap()),
        Err(e) => content::Json(String::from("{}")),
    }
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


#[post("/repo/plugins/download", data = "<req>")]
pub fn download_plugin(db: State<MyPool>, req: Form<DownloadReq>) -> PathResp {
    let db_ref = db.value();
    let name = &req.name.clone();
    let query = async move |tx_c: Sender<Result<Vec<PluginsStore>, my::error::Error>>| {
        let query_res = db_ref.prep_exec(
            "select id,name,location,official,enable from plugins_store where name=?", (&req.name, )).unwrap();

        let res = query_res.map(|x| x.unwrap()).map(|row| {
            let (id, name, location, official, enable) = my::from_row::<(i32, String, String, i32, i32)>(row);
            PluginsStore { id, name, location, official, enable }
        }).collect::<Vec<PluginsStore>>();
        tx_c.send(Ok(res)).unwrap();
        ()
    };
    let res: Vec<PluginsStore> = db.future_exec(query).unwrap();
    //let file = File::open(&res[0].location);
    //file.map(|x| Stream::from(x))
    let zero: usize = 0;
    if &res.len() > &zero {
        PathResp::File(PathBuf::from(&res[0].location))
    } else {
        PathResp::ErrorMessage(String::from(format!("{} is not exists", name)))
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct PluginsStore {
    id: i32,
    name: String,
    location: String,
    official: i32,
    enable: i32,
}

#[derive(FromForm)]
pub struct DownloadReq {
    name: String,
}