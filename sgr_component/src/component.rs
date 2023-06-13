use async_trait::async_trait;
use futures::channel::oneshot;
use futures::stream::BoxStream;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Args {
    fn get_one<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned + Default;
}

impl Args for HashMap<String, String> {
    fn get_one<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned + Default,
    {
        self.get(key).map(|v| {
            serde_json::from_str(v).unwrap_or_else(|_| {
                log::error!("parse config error: {}", v);
                T::default()
            })
        })
    }
}

pub trait Component: Default {
    fn init(&mut self, args: impl Args);
    fn stop(&mut self);
}

pub type Result<T> = std::io::Result<T>;
pub type ResultReceiver<T> = Result<oneshot::Receiver<T>>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ServiceEntry {
    pub id: String,
    pub name: String,
    pub alias: Option<String>,
    pub endpoints: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[async_trait]
pub trait ServiceRegistry {
    async fn find(&self, service_name: String, id: String) -> Result<ServiceEntry>;
    async fn lookup(&self, service_name: String) -> Result<Vec<ServiceEntry>>;
    fn watch(&self, service_name: String) -> BoxStream<'_, Result<Vec<ServiceEntry>>>;
}

#[async_trait]
pub trait Cofigurator {
    async fn read_config<T>(&self, path: String) -> Result<T>
    where
        T: DeserializeOwned + Default;

    async fn watch_config<T>(&self, path: String) -> ResultReceiver<T>
    where
        T: DeserializeOwned + Default;
}
