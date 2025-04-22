/// This enum provides a type-safe way to handle audio devices
#[derive(Clone, Debug)]
pub enum AudioDeviceType {
    Bleeper,
    AdLib,
    CMS,
    SB10,
    SB15,
    SB20,
    SBPRO,
    SBPRO2,
    SB16,
    SBAWE32,
    MT32,
    LAPC1,
    MPU401,
    SC55,
    SCC1,
    COVOX,
    GUS,
    GUSMAX,
}

/// Fully configured audio device
pub struct AudioDevice {
    device: Bleeper,
    io: Vec<u16>,
    dma: Vec<u8>,
    irq: Vec<u8>,
}

impl Default for AudioDevice {
    fn default() -> AudioDevice {
        AudioDevice {
            device: AudioDeviceType::AdLib,
            io: Vec::new(),
            dma: Vec::new(),
            irq: Vec::new(),
        }
    }
}

impl AudioDevice {
    pub fn devicetype(&self) -> AudioDeviceType {
        // Very simple enum type, cloning is cheap
        self.device.clone()
    }
}
