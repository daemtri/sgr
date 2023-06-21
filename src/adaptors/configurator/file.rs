use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};

use crate::ports::component::config::Configurator;
use crate::ports::component::{Component, Result, Stream};
use futures::stream;
use tokio::fs;
use tokio::sync::watch;

pub struct FileConfigurator {
    path: String,
}

impl Component for FileConfigurator {
    fn init(args: impl crate::ports::component::Args) -> Self {
        Self {
            path: String::from("config.json"),
        }
    }
    fn stop(&mut self) {}
}

#[async_trait::async_trait]
impl Configurator for FileConfigurator {
    async fn read_config<T: Default>(&self, path: String) -> Result<T> {
        unimplemented!()
    }
    async fn watch_config<'a, T: Default + Send + 'a>(&self, path: String) -> Stream<'a, T> {
        let (tx, mut rx) = watch::channel(String::new());
        let read_result = fs::read_to_string(self.path.clone()).await;
        if let Ok(content) = read_result {
            tx.send(content).unwrap();
        }
        let path = Arc::new(self.path.clone());
        let y = stream::unfold(T::default(), move |mut state| {
            let file_path = path.clone();
            async move {
                let metadata = match fs::metadata(file_path.as_str()).await {
                    Ok(metadata) => metadata,
                    Err(e) => panic!("failed to read metadata: {}", e),
                };

                let modified = metadata.modified().unwrap();
                let next = T::default();
                Some((Ok(state), next))
            }
        });
        Box::pin(y)
    }
}
