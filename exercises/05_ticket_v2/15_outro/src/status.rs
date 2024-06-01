// Done: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
impl TryFrom<String> for Status {
	type Error = &'static str;

	fn try_from(s: String) -> Result<Status, Self::Error>{
		let s1 = s.to_lowercase();
		if s1=="todo" {
			return Ok(Status::ToDo);
		} else if s1=="inprogress" {
			return Ok(Status::InProgress);
		} else if s1=="done" {
			return Ok(Status::Done);
		} else {
			Err("Invalid status")
		}
	}
}
impl TryFrom<&str> for Status {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Status, Self::Error>{
		Status::try_from(s.to_string())
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
