use media_storage::files::{host::fs::FsFileHost, storage::Storage};

static BYTES: &[u8] = b"hello world";

#[tokio::test]
async fn test_put() {
    let dir = tempfile::tempdir().unwrap();

    let fs_host = FsFileHost::new(dir.path());
    fs_host.init().unwrap();

    let storage = Storage::new(fs_host);

    let res = storage.put(BYTES).await.unwrap();
    dbg!(res);

    let res = storage.put(BYTES).await.unwrap();
    dbg!(res);
}
