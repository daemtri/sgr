use crate::component;
use async_trait::async_trait;
use std::io::Result;

pub struct Files {}

impl Files {
    pub fn new() -> Self {
        Files {}
    }
}

#[async_trait]
impl component::ServiceRegistry for Files {
    async fn find(&self, service_name: String, id: String) -> Result<component::ServiceEntry> {
        unimplemented!()
    }
    async fn lookup(&self, service_name: String) -> Result<Vec<component::ServiceEntry>> {
        unimplemented!()
    }
    async fn watch(
        &self,
        service_name: String,
    ) -> component::ResultReceiver<Vec<component::ServiceEntry>> {
        unimplemented!()
    }
}
