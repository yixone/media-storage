use ms_blob_host::BlobHost;
use ms_content_store::{ContentStorage, key::StorageKey};
use sha2::{Digest, Sha256};
use tempfile::TempDir;

fn temp_content_store() -> ContentStorage {
    let temp = TempDir::new().unwrap();
    let backend = BlobHost::mount_fs(temp.path()).unwrap();
    ContentStorage::new(backend, 128 * 1024)
}

const TEST_BLOB: &[u8] = &[67, 42];

#[tokio::test]
async fn put_blob_and_return_result() {
    let store = temp_content_store();

    let input_sha256 = Sha256::digest(TEST_BLOB).into();

    let res = store.put(TEST_BLOB).await.unwrap();

    assert_eq!(res.key, StorageKey::from_digest(input_sha256));
    assert_eq!(res.size, TEST_BLOB.len());
    assert!(res.is_new);
}
