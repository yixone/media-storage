/// Extends query-builder to generate an update query with optional fields
macro_rules! opt_update_query {
    (
        $qb: ident, $($query:expr => $value:expr),+
    ) => {
        {
            let mut sep = $qb.separated(",");
            let mut has_changes = false;
            $(
                match &$value {
                    $crate::models::types::UpdateField::Set(v) => {
                        has_changes = true;

                        sep.push(concat!($query, "="));
                        sep.push_bind_unseparated(v);
                    },
                    $crate::models::types::UpdateField::Skip => {},

                }
            )+
            has_changes
        }
    };
}
