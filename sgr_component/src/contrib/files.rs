use crate::component::{Component, Result, ServiceEntry, ServiceRegistry};
use async_trait::async_trait;
use futures::stream::BoxStream;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct Files {
    apps_file: String,
    services: HashMap<String, ServiceEntry>,
}

impl Default for Files {
    fn default() -> Self {
        Files {
            apps_file: String::from("./configs/apps.yaml"),
            services: HashMap::new(),
        }
    }
}

impl Component for Files {
    fn init(&mut self, args: impl crate::Args) {
        if let Some(apps_file) = args.get_one::<String>("apps_file") {
            self.apps_file = apps_file;
        }
        let mut apps_file = File::open(&self.apps_file).expect("read file apps.yaml failed");
        let mut yaml_str = String::new();
        apps_file
            .read_to_string(&mut yaml_str)
            .expect("read file error");
        let apps_config: Vec<ServiceEntry> =
            serde_yaml::from_str(&yaml_str).expect("parse yaml failed");

        log::info!("AppsConfig: {:#?}", apps_config);

        let mut apps_map = HashMap::<String, ServiceEntry>::new();
        for (_, item) in apps_config.iter().enumerate() {
            apps_map.insert(item.id.clone(), item.clone());
        }

        self.services = apps_map;
    }

    fn stop(&mut self) {}
}

#[async_trait]
impl ServiceRegistry for Files {
    async fn find(&self, service_name: String, id: String) -> Result<ServiceEntry> {
        self.services
            .get(&id)
            .filter(|v| v.name == service_name)
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))
            .cloned()
    }

    async fn lookup(&self, service_name: String) -> Result<Vec<ServiceEntry>> {
        let ret: Vec<ServiceEntry> = self
            .services
            .iter()
            .filter(|(_, v)| v.name == service_name)
            .map(|(_, v)| v.clone())
            .collect();
        Ok(ret)
    }

    fn watch(&self, service_name: String) -> BoxStream<'_, Result<Vec<ServiceEntry>>> {
        Box::pin(WatchStream {})
    }
}

pub struct WatchStream {}

impl futures::stream::Stream for WatchStream {
    type Item = Result<Vec<ServiceEntry>>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        unimplemented!()
    }
}
