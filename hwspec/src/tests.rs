#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use std::str::FromStr;

    #[test]
    fn valid_8088() {
        let reference = Cpu::I8088;
        for element in ["i8088", "I8088", "8088"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_8086() {
        let reference = Cpu::I8086;
        for element in ["i8086", "I8086", "8086"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80186() {
        let reference = Cpu::I80186;
        for element in ["i80186", "I80186", "80186", "186"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80286() {
        let reference = Cpu::I80286;
        for element in ["i80286", "I80286", "286"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80386_dx() {
        let reference = Cpu::I80386DX;
        for element in ["i80386DX", "I80386DX", "80386DX", "80386", "386", "386DX"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_386_sx() {
        let reference = Cpu::I80386SX;
        for element in ["i80386SX", "I80386SX", "80386SX", "386SX"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn ambiguous_386() {
        let sx = Cpu::I80386SX;
        let dx = Cpu::I80386DX;
        let test = Cpu::from_str("386").unwrap();
        assert_ne!(test, sx);
        assert_eq!(test, dx);
    }

    #[test]
    fn valid_80486_dx() {
        let reference = Cpu::I80486DX;
        for element in ["I80486DX", "I80486", "80486DX", "80486", "486DX", "486"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80486_sx() {
        let reference = Cpu::I80486SX;
        for element in [
            "i80486sx", "I80486SX", "i80486Sx", "80486sx", "486sx", "486SX",
        ] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_sl() {
        let reference = Cpu::I80486SL;
        for element in ["i80486sl", "I80486SL", "80486sL", "486sl", "486SL"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_sx2() {
        let reference = Cpu::I80486SX2;
        for element in [
            "I80486SX2",
            "80486SX2",
            "486SX2",
            "i80486sx2",
            "80486sX2",
            "486Sx2",
        ] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_dx2() {
        let reference = Cpu::I80486DX2;
        for element in ["I80486DX2", "80486DX2", "486DX2", "i80486Dx2", "486dx2"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_dx4() {
        let reference = Cpu::I80486DX4;
        for element in ["I80486DX4", "80486DX4", "486DX4", "i80486Dx4", "486dx4"] {
            assert_eq!(reference, Cpu::from_str(element).unwrap());
        }
    }

    #[test]
    fn ambiguous_486() {
        let dx = Cpu::I80486DX;
        let ambiguous = Cpu::from_str("486").unwrap();
        assert_eq!(dx, ambiguous);
    }

    #[test]
    fn invalid_cpu_name() {
        let invalid_cpu = "MOS6502";
        assert!(Cpu::from_str(invalid_cpu).is_err());
    }

    #[test]
    fn empty_string() {
        assert!(Cpu::from_str("").is_err());
    }
}
