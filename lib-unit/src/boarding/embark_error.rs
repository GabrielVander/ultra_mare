#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbarkError {
    NotACarrier,
    CannotBeCarried,
    NotAdjacent,
    NoSpace,
}
