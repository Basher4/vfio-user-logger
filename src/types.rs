use std::str::FromStr;
use thiserror::Error;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Sbdf {
    segment: u16,
    bus: u8,
    dev: u8,
    func: u8,
}

impl std::fmt::Display for Sbdf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:04x}:{:02x}:{:02x}.{}",
            self.segment, self.bus, self.dev, self.func
        )
    }
}

#[derive(Debug, Clone, Error)]
pub enum SbdfParseError {
    #[error("{val}: missing field {field}")]
    MissingField { val: String, field: String },

    #[error("{val}: {field} is not a number")]
    NotANumber { val: String, field: String },

    #[error("{val}: {field} is out of valid range")]
    OutOfRange { val: String, field: String },
}

impl SbdfParseError {
    fn missing_field(s: &str, f: &str) -> Self {
        Self::MissingField {
            val: s.to_string(),
            field: f.to_string(),
        }
    }

    fn not_a_number(s: &str, f: &str) -> Self {
        Self::NotANumber {
            val: s.to_string(),
            field: f.to_string(),
        }
    }

    fn out_of_range(s: &str, f: &str) -> Self {
        Self::OutOfRange {
            val: s.to_string(),
            field: f.to_string(),
        }
    }
}

impl FromStr for Sbdf {
    type Err = SbdfParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sbdf_parts = s.split('.');

        let func: u64 = sbdf_parts
            .next_back()
            .ok_or_else(|| SbdfParseError::missing_field(s, "func"))
            .and_then(|x| u64::from_str_radix(x, 16).map_err(|_| SbdfParseError::not_a_number(s, "func")))?;

        let sbd_parts = sbdf_parts
            .next_back()
            .ok_or_else(|| SbdfParseError::missing_field(s, "dev"))?;
        let mut sbd_parts = sbd_parts.split(':');

        let dev: u64 = sbd_parts
            .next_back()
            .ok_or_else(|| SbdfParseError::missing_field(s, "dev"))
            .and_then(|x| u64::from_str_radix(x, 16).map_err(|_| SbdfParseError::not_a_number(s, "dev")))?;
        let bus: u64 = sbd_parts.next_back().map_or(Ok(0), |x| {
            u64::from_str_radix(x, 16).map_err(|_| SbdfParseError::not_a_number(s, "bus"))
        })?;
        let segment: u64 = sbd_parts.next_back().map_or(Ok(0), |x| {
            u64::from_str_radix(x, 16).map_err(|_| SbdfParseError::not_a_number(s, "slot"))
        })?;

        if func > 7 {
            return Err(SbdfParseError::out_of_range(s, "func"));
        }
        if dev > 31 {
            return Err(SbdfParseError::out_of_range(s, "dev"));
        }
        if bus > 255 {
            return Err(SbdfParseError::out_of_range(s, "bus"));
        }
        if segment > (u16::MAX as u64) {
            return Err(SbdfParseError::out_of_range(s, "segment"));
        }

        Ok(Self {
            segment: segment as u16,
            bus: bus as u8,
            dev: dev as u8,
            func: func as u8,
        })
    }
}
