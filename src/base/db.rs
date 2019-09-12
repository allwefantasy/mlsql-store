use crate::base::config::Config;
use mysql as my;
use std::error::Error;
use std::future::Future as StdFuture;
use std::sync::mpsc::{channel, Sender};

pub struct MyPool(my::Pool);

impl MyPool {
    pub fn new(config: &Config) -> MyPool {
        let conf_t = config.value();
        let db_config = conf_t.get("database").unwrap().as_table().unwrap();
        let user = db_config.get("user").unwrap().as_str().unwrap();
        let password = db_config.get("password").unwrap().as_str().unwrap();
        let host = db_config.get("host").unwrap().as_str().unwrap();
        let name = db_config.get("name").unwrap().as_str().unwrap();
        let port = db_config.get("port").unwrap().as_integer().unwrap();

        let mut builder = my::OptsBuilder::default();
        builder
            .user(Some(user))
            .pass(Some(password))
            .ip_or_hostname(Some(host))
            .tcp_port(port as u16)
            .db_name(Some(name));
        let pool = my::Pool::new(builder).unwrap();
        MyPool(pool)
    }

    pub fn value(&self) -> my::Pool {
        self.0.clone()
    }

    pub fn future_exec<F, R, T>(&self, callback: F) -> Result<T, my::error::Error>
    where
        R: StdFuture<Output = ()>,
        F: FnOnce(Sender<Result<T, my::error::Error>>) -> R,
    {
        let (tx, rx) = channel();
        let tx_c = tx.clone();
        let wow = callback(tx_c);
        futures::executor::block_on(wow);
        rx.recv().unwrap()
    }
}
