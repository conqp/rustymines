#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
    Continue,
    InvalidPosition,
    Lost,
    Won,
}
