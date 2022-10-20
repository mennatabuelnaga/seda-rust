pub trait DatabaseAdapter {
    fn set(&mut self, key: &str, value: &str);
    fn get(&self, key: &str);
}

pub struct Adapters {
    pub database: Box<dyn DatabaseAdapter + Send>,
}
