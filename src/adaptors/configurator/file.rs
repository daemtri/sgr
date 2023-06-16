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
    async fn read_config<T: Default>(&self, path: String) -> Result<T> {
        unimplemented!()
    }
    async fn watch_config<T: Default>(&self, path: String) -> Stream<T> {
        let y = stream::unfold(T::default(), |mut t|  {
            async move {
                let t = T::default();
                Some((Ok(t), t))
            }
        });
        Box::pin(y)
    }
}
