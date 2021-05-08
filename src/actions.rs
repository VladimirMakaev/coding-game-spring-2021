use crate::common::ParseError;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    WAIT,
    COMPLETE(u8),
    GROW(u8),
    SEED(u8, u8),
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::WAIT => write!(f, "WAIT"),
            Action::COMPLETE(x) => write!(f, "COMPLETE {}", x),
            Action::GROW(x) => write!(f, "GROW {}", x),
            Action::SEED(x, y) => write!(f, "SEED {} {}", x, y),
        }
    }
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<&str> = s.split(' ').collect();
        match (params[0], params.len()) {
            ("WAIT", 1) => Ok(Action::WAIT),
            ("WAIT", _) => Err(ParseError::InvalidParameters),
            ("COMPLETE", 2) => Ok(Action::COMPLETE(
                params[1]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
            )),
            ("GROW", 2) => Ok(Action::GROW(
                params[1]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
            )),
            ("SEED", 3) => Ok(Action::SEED(
                params[1]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
                params[2]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
            )),
            _ => Err(ParseError::UnknownInput),
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
        let result = "GROW 32".parse::<Action>();
        assert_eq!(Ok(Action::GROW(32)), result);
        let result = "SEED 32 1".parse::<Action>();
        assert_eq!(Ok(Action::SEED(32, 1)), result);
    }
}
