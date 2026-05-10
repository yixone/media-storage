#[derive(Debug, Default)]
pub enum PatchField<T> {
    Update(T),
    #[default]
    Ignore,
}

impl PatchField<Option<String>> {
    pub fn from_option_string(v: Option<String>) -> PatchField<Option<String>> {
        match v {
            Some(v) => {
                if v.trim().is_empty() {
                    PatchField::Update(None)
                } else {
                    PatchField::Update(Some(v))
                }
            }
            None => PatchField::Ignore,
        }
    }
}

impl<T> From<Option<T>> for PatchField<T> {
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => PatchField::Update(v),
            None => PatchField::Ignore,
        }
    }
}

macro_rules! patch_model {
    (
        $model_name: ident { $( $f_id: ident: $f_ty: ty ),* }
    ) => {
        #[derive(Debug, Default)]
        pub struct $model_name {
            $(pub $f_id: $crate::patch::PatchField<$f_ty> ),*
        }
    };
}
