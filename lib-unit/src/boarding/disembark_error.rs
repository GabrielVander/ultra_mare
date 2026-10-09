use crate::movement::move_error::MoveError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisembarkError {
    NobodyAboard,
    Blocked(MoveError),
}
