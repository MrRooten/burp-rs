

pub trait IObject {
    fn get_object(&self, path: &str) -> Option<String>;
}
