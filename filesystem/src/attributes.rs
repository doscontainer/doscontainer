use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Attributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    volume_label: bool,
    subdir: bool,
    archive: bool,
    device: bool,
}

pub enum AttributesPreset {
    RegularFile,
    SystemFile,
    Directory,
    VolumeLabel,
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.read_only {
            flags.push("READ_ONLY");
        }
        if self.hidden {
            flags.push("HIDDEN");
        }
        if self.system {
            flags.push("SYSTEM");
        }
        if self.volume_label {
            flags.push("VOLUME_LABEL");
        }
        if self.subdir {
            flags.push("SUBDIR");
        }
        if self.archive {
            flags.push("ARCHIVE");
        }
        if self.device {
            flags.push("DEVICE");
        }

        write!(f, "Attributes({})", flags.join(" | "))
    }
}

impl Attributes {
    pub fn from_preset(preset: AttributesPreset) -> Attributes {
        match preset {
            AttributesPreset::RegularFile => Attributes {
                read_only: false,
                hidden: false,
                system: false,
                volume_label: false,
                subdir: false,
                archive: true,
                device: false,
            },
            AttributesPreset::SystemFile => Attributes {
                read_only: false,
                hidden: true,
                system: true,
                volume_label: false,
                subdir: false,
                archive: false,
                device: false,
            },
            AttributesPreset::Directory => Attributes {
                read_only: false,
                hidden: false,
                system: false,
                volume_label: false,
                subdir: true,
                archive: false,
                device: false,
            },
            AttributesPreset::VolumeLabel => Attributes {
                read_only: false,
                hidden: false,
                system: false,
                volume_label: true,
                subdir: false,
                archive: false,
                device: false,
            },
        }
    }

    pub fn as_byte(&self) -> u8 {
        let mut byte = 0u8;

        // Set bits according to the FAT attribute bitfield structure
        if self.read_only {
            byte |= 0b00000001;
        }
        if self.hidden {
            byte |= 0b00000010;
        }
        if self.system {
            byte |= 0b00000100;
        }
        if self.volume_label {
            byte |= 0b00001000;
        }
        if self.subdir {
            byte |= 0b00010000;
        }
        if self.archive {
            byte |= 0b00100000;
        }
        if self.device {
            byte |= 0b01000000;
        }

        // 0 never happens in real life, so we treat it as the empty placeholder.
        // An empty placeholder holds value 0xF6 on IBM systems.
        if byte == 0 {
            0xF6
        } else {
            byte
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Attributes, AttributesPreset};

    #[test]
    fn regular_file() {
        let attrib = Attributes::from_preset(AttributesPreset::RegularFile);
        let byte = attrib.as_byte();
        assert_eq!(byte, 32);
    }

    #[test]
    fn sysfile() {
        let attrib = Attributes::from_preset(AttributesPreset::SystemFile);
        let byte = attrib.as_byte();
        assert_eq!(byte, 6);
    }

    #[test]
    fn subdir() {
        let attrib = Attributes::from_preset(AttributesPreset::Directory);
        let byte = attrib.as_byte();
        assert_eq!(byte, 0x10);
    }
}
