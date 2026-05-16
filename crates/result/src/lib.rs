pub mod error;
pub mod macros;

pub use error::AppError;

fn t() {
    let e = std::io::Error::other("FOO");
    create_error!(source = e);
    create_error!(MediaTooLarge { size: 15, max: 14 });
}
