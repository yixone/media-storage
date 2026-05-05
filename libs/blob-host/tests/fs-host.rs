use bytes::Bytes;
use ms_blob_host::{BlobHostExt, BlobPath, FsBlobHost};
use tempfile::TempDir;
use tokio::io::AsyncReadExt;

fn temp_blob_host() -> FsBlobHost {
    let temp = TempDir::new().unwrap();
    FsBlobHost::new(temp.path()).unwrap()
}

const TEST_BLOB: Bytes = Bytes::from_static(&[67, 42]);

#[tokio::test]
async fn write_and_finalize_the_blob() {
    let host = temp_blob_host();

    let path = BlobPath::new("test/file.bin");
    let mut writer = host.open_writer(&path).await.unwrap();
    writer.write(TEST_BLOB).await.unwrap();
    writer.finalize().await.unwrap();

    let mut reader = host.get_reader(&path).await.unwrap();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await.unwrap();

    assert_eq!(
        TEST_BLOB, buf,
        "The resulting blob must match the one written using the same key"
    );
}
