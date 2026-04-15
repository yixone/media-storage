/// The field state to update
/// - [`UpdateField::Skip`] - skips updating the field
/// - [`UpdateField::Set`] - sets the field to the passed value
#[derive(Debug, PartialEq, Default)]
pub enum UpdateField<T> {
    #[default]
    Skip,
    Set(T),
}
