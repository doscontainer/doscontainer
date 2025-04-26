#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::cpu::Cpu;

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
        for element in ["i80286", "I80286","286"] {
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
}