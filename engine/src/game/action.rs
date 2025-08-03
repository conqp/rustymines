use grid2d::Coordinate;

/// Possible player actions during a game.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Visit the field at the given coordinate.
    Visit(Coordinate),
    /// Toggle the flag on the field at the given coordinate.
    ToggleFlag(Coordinate),
    /// Visit all non-flagged fields.
    VisitAllNonFlaggedFields,
}
