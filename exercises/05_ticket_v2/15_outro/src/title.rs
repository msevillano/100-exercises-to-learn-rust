// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

#[derive(Debug, thiserror::Error)]
pub enum TitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

fn validate(text: &str) -> Result<(), TitleError> {
    let string_length = text.len();

    match string_length {
        1..=50 => Ok(()),
        0 => Err(TitleError::Empty),
        _ => Err(TitleError::TooLong),
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TitleError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(self::TicketTitle(value.to_string()))
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TitleError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(self::TicketTitle(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
