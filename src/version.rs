use serde_derive::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: Option<u8>,
}

impl Version {
    pub fn match_major_minor(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        if let None = self.patch {
            return format!("{}.{}", self.major, self.minor);
        }

        format!("{}.{}.{}", self.major, self.minor, self.patch.unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseVersionError {
    Empty,
    ParseInt(ParseIntError),
    BadLength,
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(ParseVersionError::Empty);
        }

        let mut parts = s.split(".");
        let part_count = parts.clone().count();
        if part_count < 2 || part_count > 3 {
            return Err(ParseVersionError::BadLength);
        }

        let major = parts.next().unwrap().parse::<u8>();

        if let Err(err) = major {
            return Err(ParseVersionError::ParseInt(err));
        }

        let minor = parts.next().unwrap().parse::<u8>();

        if let Err(err) = minor {
            return Err(ParseVersionError::ParseInt(err));
        }

        let patch = parts.next();

        if let None = patch {
            return Ok(Version {
                major: major.unwrap(),
                minor: minor.unwrap(),
                patch: None,
            });
        }

        let patch = patch.unwrap().parse::<u8>();

        match patch {
            Err(err) => Err(ParseVersionError::ParseInt(err)),
            Ok(patch) => Ok(Version {
                major: major.unwrap(),
                minor: minor.unwrap(),
                patch: Some(patch),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::version::{ParseVersionError, Version};

    #[test]
    fn test_parse_string_short() {
        assert_eq!(
            Ok(Version {
                major: 7,
                minor: 4,
                patch: None,
            }),
            "7.4".parse::<Version>()
        )
    }

    #[test]
    fn test_parse_string_long() {
        assert_eq!(
            Ok(Version {
                major: 7,
                minor: 4,
                patch: Some(9),
            }),
            "7.4.9".parse::<Version>()
        )
    }

    #[test]
    fn test_parse_string_fail_int() {
        assert!(matches!(
            "A.B".parse::<Version>(),
            Err(ParseVersionError::ParseInt(_))
        ));
    }

    #[test]
    fn test_parse_string_fail_empty() {
        assert_eq!(Err(ParseVersionError::Empty), "".parse::<Version>())
    }

    #[test]
    fn test_parse_string_fail_only_major() {
        assert_eq!(
            Err(ParseVersionError::BadLength),
            "7.4.8.5".parse::<Version>()
        )
    }

    #[test]
    fn test_parse_string_fail_patch() {
        assert_eq!(Err(ParseVersionError::BadLength), "8".parse::<Version>())
    }

    #[test]
    fn test_major_minor_true() {
        assert!(Version {
            major: 8,
            minor: 1,
            patch: Some(2),
        }
        .match_major_minor(&Version {
            major: 8,
            minor: 1,
            patch: None,
        }))
    }

    #[test]
    fn test_major_minor_false() {
        assert!(!Version {
            major: 8,
            minor: 1,
            patch: Some(2),
        }
        .match_major_minor(&Version {
            major: 8,
            minor: 0,
            patch: None,
        }))
    }
}
