#[cfg(test)]
mod tests {
    use crate::hwspec::{
        audio::AudioDevice,
        cpu::{Cpu, CpuFamily},
        storage::Floppy,
        HwSpec,
    };
    use std::io::Write;
    use std::str::FromStr;
    use tempfile::NamedTempFile;

    #[test]
    fn valid_8088() {
        let reference = CpuFamily::I8088;
        for element in ["i8088", "I8088", "8088"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_8086() {
        let reference = CpuFamily::I8086;
        for element in ["i8086", "I8086", "8086"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80186() {
        let reference = CpuFamily::I80186;
        for element in ["i80186", "I80186", "80186", "186"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80286() {
        let reference = CpuFamily::I80286;
        for element in ["i80286", "I80286", "286"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80386_dx() {
        let reference = CpuFamily::I80386DX;
        for element in ["i80386DX", "I80386DX", "80386DX", "80386", "386", "386DX"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_386_sx() {
        let reference = CpuFamily::I80386SX;
        for element in ["i80386SX", "I80386SX", "80386SX", "386SX"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn ambiguous_386() {
        let sx = CpuFamily::I80386SX;
        let dx = CpuFamily::I80386DX;
        let test = CpuFamily::from_str("386").unwrap();
        assert_ne!(test, sx);
        assert_eq!(test, dx);
    }

    #[test]
    fn valid_80486_dx() {
        let reference = CpuFamily::I80486DX;
        for element in ["I80486DX", "I80486", "80486DX", "80486", "486DX", "486"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_80486_sx() {
        let reference = CpuFamily::I80486SX;
        for element in [
            "i80486sx", "I80486SX", "i80486Sx", "80486sx", "486sx", "486SX",
        ] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_sl() {
        let reference = CpuFamily::I80486SL;
        for element in ["i80486sl", "I80486SL", "80486sL", "486sl", "486SL"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_sx2() {
        let reference = CpuFamily::I80486SX2;
        for element in [
            "I80486SX2",
            "80486SX2",
            "486SX2",
            "i80486sx2",
            "80486sX2",
            "486Sx2",
        ] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_dx2() {
        let reference = CpuFamily::I80486DX2;
        for element in ["I80486DX2", "80486DX2", "486DX2", "i80486Dx2", "486dx2"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn valid_486_dx4() {
        let reference = CpuFamily::I80486DX4;
        for element in ["I80486DX4", "80486DX4", "486DX4", "i80486Dx4", "486dx4"] {
            assert_eq!(reference, CpuFamily::from_str(element).unwrap());
        }
    }

    #[test]
    fn ambiguous_486() {
        let dx = CpuFamily::I80486DX;
        let ambiguous = CpuFamily::from_str("486").unwrap();
        assert_eq!(dx, ambiguous);
    }

    #[test]
    fn invalid_cpu_name() {
        let invalid_cpu = "MOS6502";
        assert!(CpuFamily::from_str(invalid_cpu).is_err());
    }

    #[test]
    fn empty_string() {
        assert!(CpuFamily::from_str("").is_err());
    }

    #[test]
    fn illegal_overclock() {
        let mut i386 = Cpu::from_str("386").unwrap();
        assert!(i386.set_clock(150).is_err());
    }

    #[test]
    fn illegal_underclock() {
        let mut i486 = Cpu::from_str("486").unwrap();
        assert!(i486.set_clock(4).is_err());
    }

    #[test]
    fn legitimate_386dx_clocks() {
        let mut i386dx = Cpu::from_str("386").unwrap();
        for clock in [16, 20, 25, 33, 40] {
            assert!(i386dx.set_clock(clock).is_ok());
            assert_eq!(i386dx.clock(), clock);
        }
    }

    #[test]
    fn create_valid_floppy_drives() {
        // These are the shortest-form floppy type strings we'll accept here.
        for element in ["160", "180", "320", "360", "1200", "1440", "720", "2880"] {
            assert!(Floppy::from_str(element).is_ok())
        }
    }

    #[test]
    fn valid_audio_from_string() {
        for device in [
            "bleeper",
            "speaker",
            "pcspeaker",
            "pc speaker",
            "adlib",
            "cms",
            "gameblaster",
            "game blaster",
            "sb10",
            "sb15",
            "sb20",
            "sbpro",
            "sbpro2",
            "sbpro20",
            "sb16",
            "sbawe32",
            "awe32",
            "mt32",
            "mt-32",
            "lapc1",
            "lapci",
            "lapc-1",
            "lapc-i",
            "mpu401",
            "mpu-401",
            "sc55",
            "sc-55",
            "scc1",
            "covox",
            "disney",
            "gus",
            "ultrasound",
            "gusmax",
            "ultrasoundmax",
            "ultrasound max",
        ] {
            let device = AudioDevice::from_str(device);
            assert!(device.is_ok());
        }
    }

    #[test]
    fn invalid_audio_device() {
        assert!(AudioDevice::from_str("invalid").is_err());
    }

    #[test]
    fn load_toml_from_file() {
        let toml_string = r#"
ram = "512k"
video = "vga"

[cpu]
family = "8088"
clock = 4

[[audio]]
device = "bleeper"

[[audio]]
device = "gus"
"#;

        // Create a temporary file and write the TOML to it
        let mut file = NamedTempFile::new().expect("Failed to create temp file");
        write!(file, "{}", toml_string).expect("Failed to write to temp file");

        // Load the spec from the file path
        let spec = HwSpec::from_toml(file.path());
        println!("{:?}", spec);
        assert!(spec.is_ok());
    }
}
