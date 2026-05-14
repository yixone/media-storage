use asset_shelf_storage::{ContentStorage, StorageKey, blob_host::fs::FsBlobHost};
use sha2::{Digest, Sha256};
use tempfile::TempDir;

const TEST_BLOB: &[u8] = &[67, 42];

fn temp_content_store() -> ContentStorage<FsBlobHost> {
    let temp = TempDir::new().unwrap();
    let backend = FsBlobHost::new(temp.path()).unwrap();
    ContentStorage::new(backend, 128 * 1024)
}

#[tokio::test]
async fn put_blob_and_return_result() {
    let store = temp_content_store();

    let input_sha256 = Sha256::digest(TEST_BLOB).into();

    let res = store.put(TEST_BLOB).await.unwrap();

    assert_eq!(res.key, StorageKey::from_digest(&input_sha256));
    assert_eq!(res.size, TEST_BLOB.len());
    assert!(res.is_new);
}
