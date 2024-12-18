
pub struct ReverseItem {

}

pub trait IReverse {
    fn get_url(&self) -> String;

    fn retrieve(&self, url: &str) -> Vec<ReverseItem>;
}