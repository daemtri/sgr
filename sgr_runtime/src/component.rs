use async_trait::async_trait;
use futures::channel::oneshot;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
pub trait Component {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn metadata(&self) -> HashMap<String, String>;
}

pub type Result<T> = std::io::Result<T>;
pub type ResultReceiver<T> = Result<oneshot::Receiver<T>>;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ServiceEntry {
    id: String,
    name: String,
    alias: Option<String>,
    endpoints: Vec<String>,
    metadata: HashMap<String, String>,
}

#[async_trait]
pub trait ServiceRegistry {
    async fn find(&self, service_name: String, id: String) -> Result<ServiceEntry>;
    async fn lookup(&self, service_name: String) -> Result<Vec<ServiceEntry>>;
    async fn watch(&self, service_name: String) -> ResultReceiver<Vec<ServiceEntry>>;
}

#[async_trait]
pub trait Cofigurator: Component {
    async fn read_config<T>(&self, path: String) -> Result<T>
    where
        T: DeserializeOwned + Default;

    async fn watch_config<T>(&self, path: String) -> ResultReceiver<T>
    where
        T: DeserializeOwned + Default;
}
