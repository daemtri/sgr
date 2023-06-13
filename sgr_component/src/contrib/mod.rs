pub mod files;

pub enum MyServiceRegistry {
    Files(files::Files),
}
