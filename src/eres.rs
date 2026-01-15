#[derive(Debug)]
pub enum EvalResult {
    Text(String),
    Error(String),
}

impl EvalResult {
    pub fn to_string(&self) -> String {
        match self {
            Self::Text(txt) => txt.clone(),
            Self::Error(err) => err.clone(),
        }
    }
}
