#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ExpectedTransferFail,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
