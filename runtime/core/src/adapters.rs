pub trait DatabaseAdapter: Send {
    fn set(&mut self, key: &str, value: &str);
    fn get(&self, key: &str) -> Option<&String>;
}

pub trait AdapterTypes: Clone + Default + 'static {
    type Database: DatabaseAdapter;
}

#[derive(Default)]
pub struct Adapters<Types>
where
    Types: AdapterTypes,
{
    pub database: Types::Database,
}
