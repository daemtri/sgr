use crate::ports::component::{Component, Result,Stream};
use crate::ports::component::config::Configurator;
use futures::stream;

pub struct FileConfigurator {
    path: String,
}

impl Component for FileConfigurator {
    fn init(args: impl crate::ports::component::Args) ->Self {
        Self {
            path: String::from("config.json"),
        }
    }
    fn stop(&mut self) {
        
    }
}

#[async_trait::async_trait]
impl Configurator for FileConfigurator {
    async fn read_config<T>(&self, path: String) -> Result<T> {
        unimplemented!()
    }
    async fn watch_config<T>(&self, path: String) -> Stream<T> {
        unimplemented!("xxx")
    }
}

pub struct ConfigStreamWatcher {
    path: String,
}

impl<T> stream::Stream for ConfigStreamWatcher 
    where T: serde::de::DeserializeOwned {
    type Item = T;
     fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        unimplemented!()
    }
}