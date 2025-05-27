use std::{fmt, str::FromStr};

use crate::error::OsError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsVendor {
    DigitalResearch,
    FreeDOS,
    IBM,
    Microsoft,
}

impl OsVendor {
    pub fn from_product(product: &str) -> Result<Self, OsError> {
        match product.trim().to_lowercase().as_str() {
            "freedos" | "free dos" => Ok(OsVendor::FreeDOS),
            "ms-dos" | "msdos" | "dos" => Ok(OsVendor::Microsoft),
            "pc-dos" | "pcdos" | "ibmdos" => Ok(OsVendor::IBM),
            "dr-dos" | "drdos" => Ok(OsVendor::DigitalResearch),
            _ => Err(OsError::InvalidOsProduct(product.to_string())),
        }
    }
}

impl fmt::Display for OsVendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            OsVendor::DigitalResearch => "Digital Research",
            OsVendor::FreeDOS => "FreeDOS",
            OsVendor::IBM => "IBM",
            OsVendor::Microsoft => "Microsoft",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for OsVendor {
    type Err = OsError;

    fn from_str(input: &str) -> Result<Self, OsError> {
        match input.trim().to_lowercase().as_str() {
            "dr" | "dri" | "digital" | "digital research" | "digitalresearch" => {
                Ok(OsVendor::DigitalResearch)
            }
            "freedos" | "jimhall" | "jim hall" | "freedos project" => Ok(OsVendor::FreeDOS),
            "ibm" => Ok(OsVendor::IBM),
            "ms" | "microsoft" | "micro-soft" | "micro soft" => Ok(OsVendor::Microsoft),
            _ => Err(OsError::InvalidOsVendor(input.to_string())),
        }
    }
}
