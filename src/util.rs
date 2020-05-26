use crate::cli::Source;
use crate::message::scan_job::InputSource;
use crate::message::scan_status::AdfState;
use crate::scanner::ScannerError;

pub(crate) fn choose_source(
    source: Source,
    adf_state: Option<AdfState>,
) -> Result<InputSource, ScannerError> {
    match adf_state {
        Some(AdfState::Loaded) => match source {
            Source::adf | Source::auto => Ok(InputSource::Adf),
            Source::glass => Ok(InputSource::Platen),
        },
        _ => match source {
            Source::adf => Err(ScannerError::AdfEmpty),
            _ => Ok(InputSource::Platen),
        },
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_choose_source() {
        assert_eq!(
            choose_source(Source::glass, None).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::glass, Some(AdfState::Empty)).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::glass, Some(AdfState::Loaded)).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::auto, None).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::auto, Some(AdfState::Empty)).unwrap(),
            InputSource::Platen
        );
        assert_eq!(
            choose_source(Source::auto, Some(AdfState::Loaded)).unwrap(),
            InputSource::Adf
        );
        assert!(choose_source(Source::adf, None).is_err());
        assert!(choose_source(Source::adf, Some(AdfState::Empty)).is_err());
        assert_eq!(
            choose_source(Source::adf, Some(AdfState::Loaded)).unwrap(),
            InputSource::Adf
        );
    }
}
