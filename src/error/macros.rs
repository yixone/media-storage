/// Creates a new AppError with the specified type and source
#[macro_export]
macro_rules! create_error {
    ( $error_type: ident $( { $( $arg_id: ident: $arg_expr: expr ),* } )? ) => {
        $crate::error::AppError::new(
            $crate::error::ErrorType::$error_type $( { $( $arg_id: $arg_expr ),* } )?,
            None
        )
    };

    ( $error_type: ident $( { $( $arg_id: ident: $arg_expr: expr ),* } )?, $source: expr ) => {
        $crate::error::AppError::new(
            $crate::error::ErrorType::$error_type $( { $( $arg_id: $arg_expr ),* } )?,
            Some(Box::new($source))
        )
    };
}

/// Defines a mapper for an AppError
macro_rules! map_error {
    ( $original: path => $error: ident ) => {
        impl From<$original> for $crate::error::AppError {
            #[track_caller]
            fn from(e: $original) -> Self {
                $crate::create_error!($error, e)
            }
        }
    };
    ( $original: path => $mapper: expr ) => {
        impl From<$original> for $crate::error::AppError {
            #[track_caller]
            fn from(e: $original) -> Self {
                ($mapper)(e)
            }
        }
    };
}
