use std::fmt;

#[derive(Debug)]
pub enum RfamilyError {
    InvalidRuleset(String),
    PresetNotFound(String),
    GenerationFailed(String),
    IoError(std::io::Error),
}

impl fmt::Display for RfamilyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RfamilyError::InvalidRuleset(msg) => write!(f, "Invalid ruleset: {}", msg),
            RfamilyError::PresetNotFound(name) => write!(f, "Preset '{}' not found", name),
            RfamilyError::GenerationFailed(msg) => write!(f, "Generation failed: {}", msg),
            RfamilyError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for RfamilyError {}

impl From<std::io::Error> for RfamilyError {
    fn from(err: std::io::Error) -> Self {
        RfamilyError::IoError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_invalid_ruleset_display() {
        let err = RfamilyError::InvalidRuleset("missing required field".to_string());
        assert_eq!(err.to_string(), "Invalid ruleset: missing required field");
    }

    #[test]
    fn test_preset_not_found_display() {
        let err = RfamilyError::PresetNotFound("klingon".to_string());
        assert_eq!(err.to_string(), "Preset 'klingon' not found");
    }

    #[test]
    fn test_generation_failed_display() {
        let err = RfamilyError::GenerationFailed("out of memory".to_string());
        assert_eq!(err.to_string(), "Generation failed: out of memory");
    }

    #[test]
    fn test_io_error_display() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = RfamilyError::IoError(io_err);
        assert!(err.to_string().contains("IO error"));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_error_trait_implementation() {
        let err = RfamilyError::PresetNotFound("test".to_string());
        // Verify it implements std::error::Error
        let _: &dyn Error = &err;
    }

    #[test]
    fn test_from_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let rfamily_err: RfamilyError = io_err.into();

        match rfamily_err {
            RfamilyError::IoError(e) => {
                assert_eq!(e.kind(), std::io::ErrorKind::PermissionDenied);
            }
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_error_debug_format() {
        let err = RfamilyError::InvalidRuleset("test".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("InvalidRuleset"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_all_error_variants() {
        // Ensure all variants can be constructed
        let _invalid = RfamilyError::InvalidRuleset("test".to_string());
        let _not_found = RfamilyError::PresetNotFound("test".to_string());
        let _failed = RfamilyError::GenerationFailed("test".to_string());
        let _io = RfamilyError::IoError(std::io::Error::other("test"));
    }
}
