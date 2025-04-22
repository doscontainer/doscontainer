use crate::error::HwStateError;
use std::fmt;
use std::str::FromStr;

/// Represents the various CPU types we support for DOS systems.
pub enum Cpu {
    /// Intel 8086 CPU
    I8086,
    /// Intel 8088 CPU, used in the original IBM PC
    I8088,
    /// NEC V20, faster 8088 with more instructions
    NECV20,
    /// NEC V30, faster 8086 with more instructions
    NECV30,
    /// Intel 80186, often used in embedded systems (rare in consumer PCs)
    I80186,
    /// Intel 80286, used in the IBM PC/AT, with protected mode
    I80286,
    /// Intel 80386SX, 386 with a 16-bit external data bus
    I80386SX,
    /// Intel 80386DX, full 32-bit 386 CPU
    I80386DX,
    /// Intel 80486SL, mobile version with power management features
    I80486SL,
    /// Intel 80486SX, 486 with 16-bit external data bus, no FPU
    I80486SX,
    /// Intel 80486SX2, clock-doubled 486SX
    I80486SX2,
    /// Intel 80486DX, standard 486 with FPU support
    I80486DX,
    /// Intel 80486DX2, clock-doubled 486 with FPU support
    I80486DX2,
    /// Intel 80486DX4, clock-tripled 486 with FPU support
    I80486DX4,
}

impl fmt::Display for Cpu {
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
