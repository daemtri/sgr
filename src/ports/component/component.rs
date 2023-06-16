use serde::de::DeserializeOwned;
use std::{collections::HashMap, pin::Pin};
use futures::stream;

pub trait Args {
    fn get_one<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned + Default;
}

impl Args for HashMap<String, String> {
    fn get_one<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned + Default,
    {
        self.get(key).map(|v| {
            serde_json::from_str(v).unwrap_or_else(|_| {
                log::error!("parse config error: {}", v);
                T::default()
            })
        })
    }
}

pub trait Component {
    fn init(args: impl Args) -> Self;
    fn stop(&mut self);
}

pub type Result<T> = std::io::Result<T>;
pub type Stream<T> = Pin<Box<dyn stream::Stream<Item = Result<T>>>>;