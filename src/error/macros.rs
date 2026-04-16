/// Creates a new AppError with the specified type and source
#[macro_export]
macro_rules! create_error {
    ( $error_type: ident $( $tt: tt )? ) => {
        $crate::error::AppError::new(
            $crate::error::ErrorType::$error_type $( $tt )?
        )
    };
}

/// Defines a mapper for an AppError
macro_rules! map_error {
    ( $original: path => $error: ident ) => {
        impl From<$original> for $crate::error::AppError {
            #[track_caller]
            fn from(e: $original) -> Self {
                let caller = std::panic::Location::caller();
                tracing::error!(
                    source = ?e,
                    file = caller.file(),
                    line = caller.line(),
                    "Error occured: "
                );

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
