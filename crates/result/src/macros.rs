#[macro_export]
macro_rules! create_error {
    ($kind: ident $( $tt:tt )?) => {
        $crate::AppError::new( $crate::error::ErrorKind::$kind $( $tt )?);
    };

    (source = $source: expr) => {
        $crate::AppError::internal($source);
    };
}
