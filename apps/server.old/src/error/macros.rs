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

                $crate::error::AppError::$error
            }
        }
    };
    ( $original: path => $mapper: expr ) => {
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

                ($mapper)(e)
            }
        }
    };
}
