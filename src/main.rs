pub mod app;
pub mod bootstrap;
use sgr_runtime::component::{Result, ResultReceiver, ServiceEntry, ServiceRegistry};

#[derive(sgr_derive::ServiceRegistryAgent, Debug)]
pub enum MyServiceRegistry {
    #[driver(name = "files")]
    Files(FileRegistry),
}

fn main() {
    MyServiceRegistry::new("files".to_string());
    let m = MyServiceRegistry::new("files".to_string());
    println!("Hello, world!: {:#?}", m);
}

#[derive(Debug, Default)]
pub struct FileRegistry {
    path: String,
}

#[async_trait::async_trait]
impl sgr_runtime::component::ServiceRegistry for FileRegistry {
    async fn find(&self, service_name: String, id: String) -> Result<ServiceEntry> {
        unimplemented!()
    }
    async fn lookup(&self, service_name: String) -> Result<Vec<ServiceEntry>> {
        unimplemented!()
    }
    async fn watch(&self, service_name: String) -> ResultReceiver<Vec<ServiceEntry>> {
        unimplemented!()
    }
}
