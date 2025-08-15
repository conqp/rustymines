/// Possible outcomes when visiting a field.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VisitResult {
    /// The field has been cleared.
    Cleared,
    /// The field has already been visited.
    AlreadyVisited,
    /// The field cannot be visited, because it is flagged.
    Flagged,
    /// The player stepped onto a mine.
    SteppedOnMine,
    /// The player stepped onto a dud.
    SteppedOnDud,
}
