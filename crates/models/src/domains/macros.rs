macro_rules! derived_error {
    (
        $item: item
    ) => {
        #[derive(Debug)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Serialize),
            serde(rename_all = "SCREAMING_SNAKE_CASE")
        )]
        $item
    };
}
