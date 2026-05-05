use bytes::Bytes;
use ms_blob_host::{
    ext::{BlobHostExt, RemoveBlobResult},
    fs::FsBlobHost,
    path::BlobPath,
};
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

#[tokio::test]
async fn write_and_abort_does_not_create_a_file() {
    let host = temp_blob_host();

    let path = BlobPath::new("test/file.bin");
    let mut writer = host.open_writer(&path).await.unwrap();
    writer.write(TEST_BLOB).await.unwrap();
    writer.abort().await.unwrap();

    assert!(
        host.get_reader(&path).await.is_err(),
        "Receiving a blob after an interrupted write should return an error"
    );
}

#[tokio::test]
async fn renaming_file_changes_path() {
    let host = temp_blob_host();

    let path = BlobPath::new("test/file.bin");
    let mut writer = host.open_writer(&path).await.unwrap();
    writer.write(TEST_BLOB).await.unwrap();
    writer.finalize().await.unwrap();

    assert!(
        host.get_reader(&path).await.is_ok(),
        "After writing a blob, attempting to retrieve it should not return an error"
    );

    let new_path = BlobPath::new("test/hello_world.txt");
    host.rename(&path, &new_path).await.unwrap();

    assert!(
        host.get_reader(&new_path).await.is_ok(),
        "After changing the key, a blob should be returned for the new key"
    );
    assert!(
        host.get_reader(&path).await.is_err(),
        "After changing the key, the blob should not be returned to the old key"
    );
}

#[tokio::test]
async fn after_deleting_the_file_should_return_deleted() {
    let host = temp_blob_host();

    let path = BlobPath::new("test/file.bin");
    let mut writer = host.open_writer(&path).await.unwrap();
    writer.write(TEST_BLOB).await.unwrap();
    writer.finalize().await.unwrap();

    assert!(
        host.get_reader(&path).await.is_ok(),
        "After writing a blob, attempting to retrieve it should not return an error"
    );

    assert_eq!(host.remove(&path).await.unwrap(), RemoveBlobResult::Removed);

    assert!(
        host.get_reader(&path).await.is_err(),
        "After deleting the file, an error should be returned when trying to retrieve it using the key"
    );
}
