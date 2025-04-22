/// Represents a specific type of audio device typically found in MS-DOS-compatible PC systems
/// manufactured between 1980 and 1996.
///
/// This enum provides a type-safe way to handle device identification and configuration.
///
/// # Examples
///
/// ```
/// let card = AudioDeviceType::SB16;
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AudioDeviceType {
    /// Standard PC speaker (beeper)
    Bleeper,
    /// AdLib FM synthesis card (Yamaha OPL2)
    AdLib,
    /// Creative Music System (CMS / Game Blaster)
    CMS,
    /// Sound Blaster 1.0
    SB10,
    /// Sound Blaster 1.5
    SB15,
    /// Sound Blaster 2.0
    SB20,
    /// Sound Blaster Pro
    SBPRO,
    /// Sound Blaster Pro 2
    SBPRO2,
    /// Sound Blaster 16
    SB16,
    /// Sound Blaster AWE32
    SBAWE32,
    /// Roland MT-32 (LA synthesis module)
    MT32,
    /// Roland LAPC-I (internal MT-32 compatible sound card)
    LAPC1,
    /// Roland MPU-401 MIDI interface
    MPU401,
    /// Roland SC-55 Sound Canvas
    SC55,
    /// Roland SCC-1 (internal SC-55-based sound card)
    SCC1,
    /// Covox Speech Thing (parallel port audio device)
    COVOX,
    /// Gravis Ultrasound
    GUS,
    /// Gravis Ultrasound MAX
    GUSMAX,
}

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

/// Represents a fully configured instance of an audio device in a system.
///
/// This struct associates a specific `AudioDeviceType` with optional hardware
/// resource assignments (I/O port address, DMA channel, and IRQ line).
///
/// Some devices may require only an I/O port, while others might also need DMA and IRQ lines.
///
/// # Examples
///
/// ```
/// let mut gus = AudioDevice::new(AudioDeviceType::GUS);
/// gus.configure(0x240, 5, 7);
/// ```
#[derive(Debug)]
pub struct AudioDevice {
    device: AudioDeviceType,
    io: Option<u16>,
    dma: Option<u8>,
    irq: Option<u8>,
}

impl AudioDevice {
    /// Creates a new `AudioDevice` instance for the given `AudioDeviceType`.
    ///
    /// Hardware resource assignments (I/O port, DMA, IRQ) are initially unset.
    ///
    /// # Arguments
    ///
    /// * `device` - The type of audio device.
    ///
    /// # Examples
    ///
    /// ```
    /// let adlib = AudioDevice::new(AudioDeviceType::AdLib);
    /// ```
    pub fn new(device: AudioDeviceType) -> Self {
        Self {
            device,
            io: None,
            dma: None,
            irq: None,
        }
    }

    /// Returns a reference to the `AudioDeviceType` of this device.
    ///
    /// # Examples
    ///
    /// ```
    /// let device = AudioDevice::new(AudioDeviceType::SB16);
    /// assert_eq!(device.device_type(), &AudioDeviceType::SB16);
    /// ```
    pub fn device_type(&self) -> &AudioDeviceType {
        &self.device
    }

    /// Sets the I/O port address for this device.
    ///
    /// # Arguments
    ///
    /// * `io` - The I/O port address (in hexadecimal, e.g., `0x220`).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sb16 = AudioDevice::new(AudioDeviceType::SB16);
    /// sb16.set_io(0x220);
    /// ```
    pub fn set_io(&mut self, io: u16) {
        self.io = Some(io);
    }

    /// Sets the DMA channel number for this device.
    ///
    /// # Arguments
    ///
    /// * `dma` - The DMA channel number (typically 0–7).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut gus = AudioDevice::new(AudioDeviceType::GUS);
    /// gus.set_dma(5);
    /// ```
    pub fn set_dma(&mut self, dma: u8) {
        self.dma = Some(dma);
    }

    /// Sets the IRQ line number for this device.
    ///
    /// # Arguments
    ///
    /// * `irq` - The IRQ line number (typically 0–15).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut gus = AudioDevice::new(AudioDeviceType::GUS);
    /// gus.set_irq(7);
    /// ```
    pub fn set_irq(&mut self, irq: u8) {
        self.irq = Some(irq);
    }

    /// Convenience method to set all hardware resources for this device at once.
    ///
    /// # Arguments
    ///
    /// * `io` - The I/O port address.
    /// * `dma` - The DMA channel number.
    /// * `irq` - The IRQ line number.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut sb16 = AudioDevice::new(AudioDeviceType::SB16);
    /// sb16.configure(0x220, 1, 5);
    /// ```
    pub fn configure(&mut self, io: u16, dma: u8, irq: u8) {
        self.set_io(io);
        self.set_dma(dma);
        self.set_irq(irq);
    }

    /// Returns the configured I/O port address, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut adlib = AudioDevice::new(AudioDeviceType::AdLib);
    /// adlib.set_io(0x388);
    /// assert_eq!(adlib.io(), Some(0x388));
    /// ```
    pub fn io(&self) -> Option<u16> {
        self.io
    }

    /// Returns the configured DMA channel number, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut gus = AudioDevice::new(AudioDeviceType::GUS);
    /// gus.set_dma(5);
    /// assert_eq!(gus.dma(), Some(5));
    /// ```
    pub fn dma(&self) -> Option<u8> {
        self.dma
    }

    /// Returns the configured IRQ line number, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut gus = AudioDevice::new(AudioDeviceType::GUS);
    /// gus.set_irq(7);
    /// assert_eq!(gus.irq(), Some(7));
    /// ```
    pub fn irq(&self) -> Option<u8> {
        self.irq
    }
}
