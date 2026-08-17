//! Object-storage adapter (S3-compatible) implementing the blob store port.

use async_trait::async_trait;

use crate::application::ports::BlobStore;

pub struct S3BlobStore {
    bucket: String,
    endpoint: String,
}

impl S3BlobStore {
    pub fn new(bucket: String, endpoint: String) -> Self {
        Self { bucket, endpoint }
    }
}

#[async_trait]
impl BlobStore for S3BlobStore {
    async fn put(&self, _key: &str, _bytes: &[u8]) -> anyhow::Result<()> {
        todo!("PUT object into the configured bucket")
    }

    async fn get(&self, _key: &str) -> anyhow::Result<Vec<u8>> {
        todo!("GET object from the configured bucket")
    }
}
