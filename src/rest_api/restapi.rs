#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait RestAPI {
    fn start(&self, port: i32);
}
