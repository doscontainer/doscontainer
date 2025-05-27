use serde::Deserialize;

use crate::error::SpecError;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Cpu {
    family: CpuFamily,
    clock: u8,
}

impl Cpu {
    pub fn family(&self) -> &CpuFamily {
        &self.family
    }

    pub fn clock(&self) -> u8 {
        self.clock
    }

    /// Set the clock rate for your CPU. We provide a lot of leeway here, but
    /// you won't be allowed to do the physically impossible. Every CPU family
    /// has a minimum and maximum clock rate that you must respect.
    ///
    /// So yes, you can set a 27 MHz 386 and we won't complain, even if no such
    /// thing ever officially existed. But you won't be able to push it over 50 MHz
    /// into pure fantasy territory — for that, you'll need a proper 486.
    pub fn set_clock(&mut self, clock: u8) -> Result<(), SpecError> {
        if clock < self.family.min_clock() {
            return Err(SpecError::ClockTooLow);
        }
        if clock > self.family.max_clock() {
            return Err(SpecError::ClockTooHigh);
        }
        self.clock = clock;
        Ok(())
    }
}

impl FromStr for Cpu {
    type Err = SpecError;

    /// Converts a string into the corresponding `Cpu` variant.
    ///
    /// This method attempts to parse a string into a CPU variant. It supports the
    /// uppercased version of the CPU names like "I8086" or "I80386DX". If the input string
    /// does not match a valid CPU name, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice representing a CPU type (e.g., "I80386DX").
    ///
    /// # Returns
    ///
    /// * `Ok(Cpu)` - The corresponding CPU variant if the string matches a valid CPU name.
    /// * `Err(SpecError)` - An error if the string does not match any valid CPU name.
    fn from_str(input: &str) -> Result<Self, SpecError> {
        let family = CpuFamily::from_str(input)?;
        let clock = family.default_clock();
        Ok(Cpu { family, clock })
    }
}

/// Represents the various CPU families we support for DOS systems.
///
/// These CPU families correspond to processors commonly used in older DOS-compatible systems.
/// Each variant of this enum represents a different CPU model, including various Intel and NEC
/// processors that were widely used in PCs from the 1980s and 1990s.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum CpuFamily {
    /// Intel 8086 CPU, a 16-bit processor that introduced the x86 architecture.
    #[serde(rename = "8086")]
    I8086,

    /// Intel 8088 CPU, similar to the 8086 but with an 8-bit external data bus; used in the original IBM PC.
    #[serde(rename = "8088")]
    I8088,

    /// NEC V20, a compatible and faster 8088 CPU with additional instructions and better performance.
    #[serde(rename = "necv20", alias = "v20", alias = "nec v20")]
    NECV20,

    /// NEC V30, a compatible 8086 processor, similar to the 8086 but with additional instructions and higher performance.
    #[serde(rename = "necv30", alias = "v30", alias = "nec v30")]
    NECV30,

    /// Intel 80186, a more advanced version of the 8086, often used in embedded systems and industrial applications (rare in consumer PCs).
    #[serde(rename = "80186")]
    I80186,

    /// Intel 80286, used in the IBM PC/AT, it introduced protected mode allowing for more advanced memory management.
    #[serde(rename = "286", alias = "80286", alias = "i286", alias = "i80286")]
    I80286,

    /// Intel 80386SX, a 16-bit external data bus version of the 80386 processor, used in lower-end systems.
    #[serde(
        rename = "386sx",
        alias = "80386sx",
        alias = "i386sx",
        alias = "i80386sx"
    )]
    I80386SX,

    /// Intel 80386DX, the full 32-bit version of the 80386 processor, offering significant performance improvements over earlier models.
    #[serde(
        rename = "386dx",
        alias = "80386dx",
        alias = "i386dx",
        alias = "i80386dx",
        alias = "386"
    )]
    I80386DX,

    /// Intel 80486SL, a mobile version of the 80486 processor, offering power management features for portable devices.
    #[serde(
        rename = "486sl",
        alias = "i486sl",
        alias = "80486sl",
        alias = "i80486sl"
    )]
    I80486SL,

    /// Intel 80486SX, a 486 processor with a 16-bit external data bus and no integrated floating point unit (FPU).
    #[serde(rename = "486sx", alias = "80486sx", alias = "i80486sx")]
    I80486SX,

    /// Intel 80486SX2, a clock-doubled version of the 80486SX processor, offering increased performance without requiring a new socket.
    #[serde(rename = "486sx2", alias = "80486sx2", alias = "i80486sx2")]
    I80486SX2,

    /// Intel 80486DX, a 486 processor with an integrated floating point unit (FPU), offering high performance for computational tasks.
    #[serde(rename = "486dx", alias = "80486dx", alias = "i80486dx", alias = "486")]
    I80486DX,

    /// Intel 80486DX2, a clock-doubled version of the 80486DX processor, providing a significant performance boost for users.
    #[serde(
        rename = "486dx2",
        alias = "80486dx2",
        alias = "i486dx2",
        alias = "i80486dx2"
    )]
    I80486DX2,

    /// Intel 80486DX4, a clock-tripled version of the 80486DX processor, offering even greater performance for demanding applications.
    #[serde(
        rename = "486dx4",
        alias = "80486dx4",
        alias = "i486dx4",
        alias = "i80486dx4"
    )]
    I80486DX4,
}

impl CpuFamily {
    /// [TODO] These default clocks are not yet correct!
    /// The '4' in XT and older CPU classes should be read as 4.77 but using a float
    /// here serves no real purpose as a .77Mhz. deviation won't be relevant for what we do.
    pub fn default_clock(&self) -> u8 {
        match self {
            CpuFamily::I80186 => 4,
            CpuFamily::I80286 => 8,
            CpuFamily::I80386DX => 25,
            CpuFamily::I80386SX => 16,
            CpuFamily::I80486DX => 33,
            CpuFamily::I80486DX2 => 50,
            CpuFamily::I80486DX4 => 100,
            CpuFamily::I80486SL => 33,
            CpuFamily::I80486SX => 25,
            CpuFamily::I80486SX2 => 50,
            CpuFamily::I8086 => 4,
            CpuFamily::I8088 => 4,
            CpuFamily::NECV20 => 8,
            CpuFamily::NECV30 => 8,
        }
    }

    /// [TODO] These clock rates are not yet correct!
    pub fn min_clock(&self) -> u8 {
        match self {
            CpuFamily::I80186 => 4,
            CpuFamily::I80286 => 6,
            CpuFamily::I80386DX => 16,
            CpuFamily::I80386SX => 12,
            CpuFamily::I80486DX => 20,
            CpuFamily::I80486DX2 => 40,
            CpuFamily::I80486DX4 => 33,
            CpuFamily::I80486SL => 16,
            CpuFamily::I80486SX => 16,
            CpuFamily::I80486SX2 => 33,
            CpuFamily::I8086 => 4,
            CpuFamily::I8088 => 4,
            CpuFamily::NECV20 => 4,
            CpuFamily::NECV30 => 4,
        }
    }

    pub fn max_clock(&self) -> u8 {
        match self {
            CpuFamily::I80186 => 16,
            CpuFamily::I80286 => 33,
            CpuFamily::I80386DX => 50,
            CpuFamily::I80386SX => 50,
            CpuFamily::I80486DX => 90,
            CpuFamily::I80486DX2 => 100,
            CpuFamily::I80486DX4 => 133,
            CpuFamily::I80486SL => 90,
            CpuFamily::I80486SX => 50,
            CpuFamily::I80486SX2 => 100,
            CpuFamily::I8086 => 16,
            CpuFamily::I8088 => 8,
            CpuFamily::NECV20 => 16,
            CpuFamily::NECV30 => 16,
        }
    }
}
impl fmt::Display for CpuFamily {
    /// Provides a user-friendly string representation of the CPU type.
    ///
    /// This implementation formats each CPU type into a human-readable string that represents
    /// the full name of the processor, e.g., "Intel 8086" or "Intel 80486DX".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            CpuFamily::I8086 => "Intel 8086",
            CpuFamily::I8088 => "Intel 8088",
            CpuFamily::NECV20 => "NEC V20",
            CpuFamily::NECV30 => "NEC V30",
            CpuFamily::I80186 => "Intel 80186",
            CpuFamily::I80286 => "Intel 80286",
            CpuFamily::I80386SX => "Intel 80386SX",
            CpuFamily::I80386DX => "Intel 80386DX",
            CpuFamily::I80486SL => "Intel 80486SL",
            CpuFamily::I80486SX => "Intel 80486SX",
            CpuFamily::I80486SX2 => "Intel 80486SX2",
            CpuFamily::I80486DX => "Intel 80486DX",
            CpuFamily::I80486DX2 => "Intel 80486DX2",
            CpuFamily::I80486DX4 => "Intel 80486DX4",
        };
        write!(f, "{}", label)
    }
}

impl FromStr for CpuFamily {
    type Err = SpecError;

    /// Converts a string into the corresponding `Cpu` variant.
    ///
    /// This method attempts to parse a string into a CPU variant. It supports the
    /// uppercased version of the CPU names like "I8086" or "I80386DX". If the input string
    /// does not match a valid CPU name, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice representing a CPU type (e.g., "I80386DX").
    ///
    /// # Returns
    ///
    /// * `Ok(Cpu)` - The corresponding CPU variant if the string matches a valid CPU name.
    /// * `Err(SpecError)` - An error if the string does not match any valid CPU name.
    fn from_str(input: &str) -> Result<Self, SpecError> {
        match input.to_uppercase().as_str() {
            "I8086" | "8086" => Ok(CpuFamily::I8086),
            "I8088" | "8088" => Ok(CpuFamily::I8088),
            "NECV20" => Ok(CpuFamily::NECV20),
            "NECV30" => Ok(CpuFamily::NECV30),
            "I80186" | "80186" | "186" => Ok(CpuFamily::I80186), // Ok, so "186" is anachronistic, but we'll accept it
            "I80286" | "80286" | "286" => Ok(CpuFamily::I80286),
            "I80386SX" | "80386SX" | "386SX" => Ok(CpuFamily::I80386SX),
            "I80386DX" | "I80386" | "80386DX" | "80386" | "386DX" | "386" => {
                Ok(CpuFamily::I80386DX)
            } // DX is the default when a bare 386 is given
            "I80486SL" | "80486SL" | "486SL" => Ok(CpuFamily::I80486SL),
            "I80486SX" | "80486SX" | "486SX" => Ok(CpuFamily::I80486SX),
            "I80486SX2" | "80486SX2" | "486SX2" => Ok(CpuFamily::I80486SX2),
            "I80486DX" | "I80486" | "80486DX" | "80486" | "486DX" | "486" => {
                Ok(CpuFamily::I80486DX)
            } // DX is the default when a bare 486 is given
            "I80486DX2" | "80486DX2" | "486DX2" => Ok(CpuFamily::I80486DX2),
            "I80486DX4" | "80486DX4" | "486DX4" => Ok(CpuFamily::I80486DX4),
            _ => Err(SpecError::InvalidCpu),
        }
    }
}

impl fmt::Display for Cpu {
    /// Provides a user-friendly string representation of the CPU type.
    ///
    /// This implementation formats each CPU type into a human-readable string that represents
    /// the full name of the processor, e.g., "Intel 8086" or "Intel 80486DX".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}Mhz.", self.family, self.clock)
    }
}
