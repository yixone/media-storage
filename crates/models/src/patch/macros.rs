macro_rules! patch_model {
    (
        $( #[$meta: meta] )*
        $model_name: ident {
            $( $f_id: ident: $f_ty: ty ),*
        }
    ) => {
        $( #[$meta] )*
        #[derive(Debug, Default)]
        pub struct $model_name {
            $(pub $f_id: $crate::patch::PatchField<$f_ty> ),*
        }

        impl $model_name {
            #[cfg(feature = "sqlx")]
            pub fn apply_qb<'a>(&'a self, qb: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>) -> u16
            {
                let mut sep = qb.separated(",");
                let mut changes = 0;

                $(
                    match &self.$f_id {
                        $crate::patch::PatchField::Update(v) => {
                            changes += 1;

                            sep.push(concat!(stringify!($f_id), " = "));
                            sep.push_bind_unseparated(v);
                        }
                        _ => {}
                    }
                )*

                changes
            }
        }

    };
}
