use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Runable: Send + Sync {
    async fn run(&self);
    async fn stop(&self);
}

pub struct Engine {
    runables: Vec<Arc<Box<dyn Runable>>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            runables: Vec::new(),
        }
    }

    pub fn add_runable(&mut self, runable: Box<dyn Runable>) {
        self.runables.push(Arc::new(runable));
    }

    pub async fn run(&self) {
        let mut tasks = Vec::new();

        for runable in self.runables.iter() {
            let runable = runable.clone();
            let task = tokio::task::spawn({
                async move {
                    runable.run().await;
                }
            });
            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await;
        }
    }

    pub fn stop(&self) {
        unimplemented!()
    }
}
