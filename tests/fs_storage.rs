use media_storage::files::{host::FileHost, storage::Storage};

static BYTES: &[u8] = b"hello world";

#[tokio::test]
async fn test_put() {
    let dir = tempfile::tempdir().unwrap();

    let host = FileHost::fs(dir.path()).await.unwrap();
    let storage = Storage::new(host);

    let res = storage.put(BYTES).await.unwrap();
    dbg!(res);

    let res = storage.put(BYTES).await.unwrap();
    dbg!(res);
}
