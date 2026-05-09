#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}

impl Pagination {
    pub fn new(limit: u32, offset: u32) -> Self {
        Pagination { limit, offset }
    }
}
