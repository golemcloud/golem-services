use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::config::TemplateStoreLocalConfig;
use async_trait::async_trait;
use tracing::info;

#[async_trait]
pub trait TemplateObjectStore {
    async fn get(&self, object_key: &str) -> Result<Vec<u8>, Box<dyn Error>>;

    async fn put(&self, object_key: &str, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

pub struct FsTemplateObjectStore {
    root_path: String,
    object_prefix: String,
}

impl FsTemplateObjectStore {
    pub fn new(config: &TemplateStoreLocalConfig) -> Result<Self, String> {
        let root_dir = std::path::PathBuf::from(config.root_path.as_str());
        if !root_dir.exists() {
            fs::create_dir_all(root_dir.clone()).map_err(|e| e.to_string())?;
        }
        info!(
            "FS Template Object Store root: {}, prefix: {}",
            root_dir.display(),
            config.object_prefix
        );

        Ok(Self {
            root_path: config.root_path.clone(),
            object_prefix: config.object_prefix.clone(),
        })
    }

    fn get_dir_path(&self) -> PathBuf {
        let root_path = std::path::PathBuf::from(self.root_path.as_str());
        if self.object_prefix.is_empty() {
            root_path
        } else {
            root_path.join(self.object_prefix.as_str())
        }
    }
}

#[async_trait]
impl TemplateObjectStore for FsTemplateObjectStore {
    async fn get(&self, object_key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let dir_path = self.get_dir_path();

        info!("Getting object: {}/{}", dir_path.display(), object_key);

        let file_path = dir_path.join(object_key);

        if file_path.exists() {
            fs::read(file_path).map_err(|e| e.into())
        } else {
            Err("Object not found".into())
        }
    }

    async fn put(&self, object_key: &str, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let dir_path = self.get_dir_path();

        info!("Putting object: {}/{}", dir_path.display(), object_key);

        if !dir_path.exists() {
            fs::create_dir_all(dir_path.clone())?;
        }

        let file_path = dir_path.join(object_key);

        fs::write(file_path, data).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::TemplateStoreLocalConfig;
    use crate::service::template_object_store::{FsTemplateObjectStore, TemplateObjectStore};

    #[tokio::test]
    pub async fn test_fs_object_store() {
        let config = TemplateStoreLocalConfig {
            root_path: "/tmp/cloud-service".to_string(),
            object_prefix: "prefix".to_string(),
        };

        let store = FsTemplateObjectStore::new(&config).unwrap();

        let object_key = "test_object";

        let data = b"hello world".to_vec();

        store.put(object_key, data).await.unwrap();

        let data = store.get(object_key).await.unwrap();

        assert_eq!(data, data);
    }
}
