use error::Result;
use super::Workspace;

pub struct LocalData();

impl Workspace for LocalData {
    fn exists(&self, name: &str) -> Result<Option<String>> {
        println!("its alive! {}", name);
        Ok(Some(name.to_string()))
    }
}
