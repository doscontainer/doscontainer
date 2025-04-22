use crate::error::HwStateError;
use std::fmt;
use std::str::FromStr;

/// Represents the various CPU types we support for DOS systems.
///
/// These CPU types correspond to processors commonly used in older DOS-compatible systems.
/// Each variant of this enum represents a different CPU model, including various Intel and NEC
/// processors that were widely used in PCs from the 1980s and 1990s.
pub enum Cpu {
    /// Intel 8086 CPU, a 16-bit processor that introduced the x86 architecture.
    I8086,

    /// Intel 8088 CPU, similar to the 8086 but with an 8-bit external data bus; used in the original IBM PC.
    I8088,

    /// NEC V20, a compatible and faster 8088 CPU with additional instructions and better performance.
    NECV20,

    /// NEC V30, a compatible 8086 processor, similar to the 8086 but with additional instructions and higher performance.
    NECV30,

    /// Intel 80186, a more advanced version of the 8086, often used in embedded systems and industrial applications (rare in consumer PCs).
    I80186,

    /// Intel 80286, used in the IBM PC/AT, it introduced protected mode allowing for more advanced memory management.
    I80286,

    /// Intel 80386SX, a 16-bit external data bus version of the 80386 processor, used in lower-end systems.
    I80386SX,

    /// Intel 80386DX, the full 32-bit version of the 80386 processor, offering significant performance improvements over earlier models.
    I80386DX,

    /// Intel 80486SL, a mobile version of the 80486 processor, offering power management features for portable devices.
    I80486SL,

    /// Intel 80486SX, a 486 processor with a 16-bit external data bus and no integrated floating point unit (FPU).
    I80486SX,

    /// Intel 80486SX2, a clock-doubled version of the 80486SX processor, offering increased performance without requiring a new socket.
    I80486SX2,

    /// Intel 80486DX, a 486 processor with an integrated floating point unit (FPU), offering high performance for computational tasks.
    I80486DX,

    /// Intel 80486DX2, a clock-doubled version of the 80486DX processor, providing a significant performance boost for users.
    I80486DX2,

    /// Intel 80486DX4, a clock-tripled version of the 80486DX processor, offering even greater performance for demanding applications.
    I80486DX4,
}

impl fmt::Display for Cpu {
    /// Provides a user-friendly string representation of the CPU type.
    ///
    /// This implementation formats each CPU type into a human-readable string that represents
    /// the full name of the processor, e.g., "Intel 8086" or "Intel 80486DX".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Cpu::I8086 => "Intel 8086",
            Cpu::I8088 => "Intel 8088",
            Cpu::NECV20 => "NEC V20",
            Cpu::NECV30 => "NEC V30",
            Cpu::I80186 => "Intel 80186",
            Cpu::I80286 => "Intel 80286",
            Cpu::I80386SX => "Intel 80386SX",
            Cpu::I80386DX => "Intel 80386DX",
            Cpu::I80486SL => "Intel 80486SL",
            Cpu::I80486SX => "Intel 80486SX",
            Cpu::I80486SX2 => "Intel 80486SX2",
            Cpu::I80486DX => "Intel 80486DX",
            Cpu::I80486DX2 => "Intel 80486DX2",
            Cpu::I80486DX4 => "Intel 80486DX4",
        };
        write!(f, "{}", label)
    }
}

impl FromStr for Cpu {
    type Err = HwStateError;

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
    /// * `Err(HwStateError)` - An error if the string does not match any valid CPU name.
    fn from_str(input: &str) -> Result<Self, HwStateError> {
        match input.to_uppercase().as_str() {
            "I8086" => Ok(Cpu::I8086),
            "I8088" => Ok(Cpu::I8088),
            "NECV20" => Ok(Cpu::NECV20),
            "NECV30" => Ok(Cpu::NECV30),
            "I80186" => Ok(Cpu::I80186),
            "I80286" => Ok(Cpu::I80286),
            "I80386SX" => Ok(Cpu::I80386SX),
            "I80386DX" => Ok(Cpu::I80386DX),
            "I80486SL" => Ok(Cpu::I80486SL),
            "I80486SX" => Ok(Cpu::I80486SX),
            "I80486SX2" => Ok(Cpu::I80486SX2),
            "I80486DX" => Ok(Cpu::I80486DX),
            "I80486DX2" => Ok(Cpu::I80486DX2),
            "I80486DX4" => Ok(Cpu::I80486DX4),
            _ => Err(HwStateError::InvalidCpu),
        }
    }
}
