use async_trait::async_trait;
use serde::de::DeserializeOwned;
use super::component::{Result,Stream};

#[async_trait]
pub trait Configurator {
    async fn read_config<T>(&self, path: String) -> Result<T>
    where
        T: DeserializeOwned + Default + Copy;

    async fn watch_config<T>(&self, path: String) -> Stream<T>
    where
        T: DeserializeOwned + Default + Copy + Send;
}
