/// Creates a new AppError with the specified type and source
#[macro_export]
macro_rules! create_error {
    ( $error_type: ident $( { $( $arg_id: ident: $arg_expr: expr ),* } )? ) => {
        $crate::error::AppError::new(
            $crate::error::ErrorType::$error_type $( { $( $arg_id: $arg_expr ),* } )?
        )
    };
}

/// Defines a mapper for an AppError
macro_rules! map_error {
    ( $original: path => $error: ident ) => {
        impl From<$original> for $crate::error::AppError {
            #[track_caller]
            fn from(_: $original) -> Self {
                $crate::create_error!($error)
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
