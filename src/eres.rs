use std::fmt::Display;

#[derive(Debug)]
pub enum EvalResult {
    Text(String),
    Error(String),
}

impl Display for EvalResult {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Text(txt) => txt.fmt(fmt),
            Self::Error(err) => err.fmt(fmt),
        }
    }
}
