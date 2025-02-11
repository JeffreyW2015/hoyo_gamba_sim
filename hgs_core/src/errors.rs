#[derive(Debug)]
pub enum PullError {
    ImpossibleRoll,
}

impl std::fmt::Display for PullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PullError::ImpossibleRoll => write!(f, "Impossible roll occured"),
        }
    }
}
