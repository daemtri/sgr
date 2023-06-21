use super::component::{Result, Stream};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Configurator {
    async fn read_config<T>(&self, path: String) -> Result<T>
    where
        T: DeserializeOwned + Default;

    async fn watch_config<'a, T>(&self, path: String) -> Stream<'a, T>
    where
        T: DeserializeOwned + Default + Send + 'a;
}
