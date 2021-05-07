use std::fmt::{Debug, Display};
use std::str::FromStr;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    WAIT,
    COMPLETE(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseActionError {
    InvalidParameters,
    UnknownAction,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::WAIT => write!(f, "WAIT"),
            Action::COMPLETE(x) => write!(f, "COMPLETE {}", x),
        }
    }
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<&str> = s.split(' ').collect();
        match (params[0], params.len()) {
            ("WAIT", 1) => Ok(Action::WAIT),
            ("WAIT", _) => Err(ParseActionError::InvalidParameters),
            ("COMPLETE", 2) => Ok(Action::COMPLETE(
                params[1]
                    .parse::<u8>()
                    .map_err(|_| ParseActionError::InvalidParameters)?,
            )),
            _ => Err(ParseActionError::UnknownAction),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_parses() {
        let result = "COMPLETE 12".parse::<Action>();
        assert_eq!(Ok(Action::COMPLETE(12)), result);
        let result = "WAIT".parse::<Action>();
        assert_eq!(Ok(Action::WAIT), result);
    }
}
