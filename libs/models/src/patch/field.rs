#[derive(Debug, Default)]
pub enum PatchField<T> {
    Update(T),

    #[default]
    Ignore,
}

impl From<Option<String>> for PatchField<Option<String>> {
    fn from(v: Option<String>) -> Self {
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
