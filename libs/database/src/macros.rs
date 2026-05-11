macro_rules! opt_update_query {
    (
        $qb: ident, $($query:expr => $value:expr),+
    ) => {
        {
            let mut sep = $qb.separated(",");
            let mut has_changes = false;
            $(
                match &$value {
                    shelf_shared_models::patch::PatchField::Update(v) => {
                        has_changes = true;

                        sep.push(concat!($query, "="));
                        sep.push_bind_unseparated(v);
                    },
                    shelf_shared_models::patch::PatchField::Ignore => {},

                }
            )+
            has_changes
        }
    };
}
