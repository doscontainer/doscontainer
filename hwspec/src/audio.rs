use serde::Deserialize;
use std::{fmt, str::FromStr};

use crate::error::HwSpecError;

/// Represents a specific type of audio device typically found in MS-DOS-compatible PC systems
/// manufactured between 1980 and 1996.
///
/// This enum provides a type-safe way to handle device identification and configuration. Some
/// effort was made to span the gamut of relevant hardware. Not *everything* that was ever released
/// is included here. Brands like TurtleBeach, Ensoniq and Aztech are missing until someone sees the
/// need to include them.
///
/// # Examples
#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
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
    /// Tandy 1000 / IBM PCjr
    Tandy,
}

impl fmt::Display for AudioDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AudioDeviceType::Bleeper => "PC Speaker",
            AudioDeviceType::AdLib => "AdLib",
            AudioDeviceType::CMS => "CMS / Game Blaster",
            AudioDeviceType::SB10 => "Sound Blaster 1.0",
            AudioDeviceType::SB15 => "Sound Blaster 1.5",
            AudioDeviceType::SB20 => "Sound Blaster 2.0",
            AudioDeviceType::SBPRO => "Sound Blaster Pro",
            AudioDeviceType::SBPRO2 => "Sound Blaster Pro 2",
            AudioDeviceType::SB16 => "Sound Blaster 16",
            AudioDeviceType::SBAWE32 => "Sound Blaster AWE32",
            AudioDeviceType::MT32 => "Roland MT-32",
            AudioDeviceType::LAPC1 => "Roland LAPC-I",
            AudioDeviceType::MPU401 => "Roland MPU-401",
            AudioDeviceType::SC55 => "Roland SC-55",
            AudioDeviceType::SCC1 => "Roland SCC-1",
            AudioDeviceType::COVOX => "Covox Speech Thing",
            AudioDeviceType::GUS => "Gravis Ultrasound",
            AudioDeviceType::GUSMAX => "Gravis Ultrasound MAX",
            AudioDeviceType::Tandy => "Tandy 1000 / IBM PCjr"
        };
        write!(f, "{}", name)
    }
}

impl FromStr for AudioDeviceType {
    type Err = HwSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AudioDeviceType::*;
        match s.trim().to_lowercase().as_str() {
            "bleeper" | "speaker" | "pcspeaker" | "pc speaker" => Ok(Bleeper),
            "tandy" | "tandy1000" | "tandy 1000" | "pcjr" | "pc jr" => Ok(Tandy),
            "adlib" => Ok(AdLib),
            "cms" | "game blaster" | "gameblaster" => Ok(CMS),
            "sb10" => Ok(SB10),
            "sb15" => Ok(SB15),
            "sb20" => Ok(SB20),
            "sbpro" => Ok(SBPRO),
            "sbpro2" => Ok(SBPRO2),
            "sb16" => Ok(SB16),
            "sbawe32" => Ok(SBAWE32),
            "mt32" => Ok(MT32),
            "lapc1" => Ok(LAPC1),
            "mpu401" => Ok(MPU401),
            "sc55" => Ok(SC55),
            "scc1" => Ok(SCC1),
            "covox" => Ok(COVOX),
            "gus" => Ok(GUS),
            "gusmax" => Ok(GUSMAX),
            _ => Err(HwSpecError::InvalidAudioDevice(s.to_string())),
        }
    }
}

/// Represents a fully configured instance of an audio device in a system.
///
/// This struct associates a specific `AudioDeviceType` with optional hardware
/// resource assignments (I/O port address, DMA channel, and IRQ line).
///
/// Some devices may require only an I/O port, while others might also need DMA and IRQ lines.
#[derive(Debug, Deserialize, PartialEq)]
pub struct AudioDevice {
    device: AudioDeviceType,
    io: Option<u16>,
    dma_low: Option<u8>,
    dma_high: Option<u8>,
    irq_low: Option<u8>,
    irq_high: Option<u8>,
}

impl AudioDevice {
    /// Creates a new `AudioDevice` instance for the given `AudioDeviceType`.
    ///
    /// Hardware resource assignments (I/O port, DMA, IRQ) are set to documented
    /// factory defaults from the original manufacturer's documentation.
    ///
    /// # Arguments
    ///
    /// * `device` - The type of audio device.
    pub fn new(device: AudioDeviceType) -> Self {
        let mut new_device = AudioDevice {
            device,
            io: None,
            dma_low: None,
            dma_high: None,
            irq_low: None,
            irq_high: None,
        };

        // Set the IO/DMA/IRQ values to sensible defaults for the device chosen
        match device {
            AudioDeviceType::Bleeper => (),
            AudioDeviceType::Tandy => (),
            AudioDeviceType::AdLib => new_device.set_io(0x388),
            AudioDeviceType::CMS => new_device.set_io(0x220),
            AudioDeviceType::SB10 | AudioDeviceType::SB15 | AudioDeviceType::SB20 => {
                new_device.set_io(0x220);
                new_device.set_dma_low(1);
                new_device.set_irq_low(7);
            }
            AudioDeviceType::SBPRO | AudioDeviceType::SBPRO2 => {
                new_device.set_io(0x220);
                new_device.set_dma_low(1);
                new_device.set_irq_low(5);
            }
            AudioDeviceType::SB16 => {
                new_device.set_io(0x220);
                new_device.set_dma_low(1);
                new_device.set_dma_high(5);
                new_device.set_irq_low(5);
                new_device.set_irq_high(11);
            }
            AudioDeviceType::SBAWE32 => {
                new_device.set_io(0x220);
                new_device.set_irq_low(5);
                new_device.set_irq_high(11);
                new_device.set_dma_low(1);
                new_device.set_dma_high(5);
            }
            AudioDeviceType::MT32
            | AudioDeviceType::LAPC1
            | AudioDeviceType::MPU401
            | AudioDeviceType::SC55
            | AudioDeviceType::SCC1 => {
                new_device.set_io(0x330);
                new_device.set_irq_low(2);
                new_device.set_irq_high(9);
            }
            AudioDeviceType::COVOX => {
                new_device.set_io(0x378);
                new_device.set_irq_low(7);
            }
            AudioDeviceType::GUS | AudioDeviceType::GUSMAX => {
                new_device.set_io(0x220);
                new_device.set_dma_high(7);
                new_device.set_dma_low(5);
                new_device.set_irq_low(5);
                new_device.set_irq_high(11);
            }
        }
        new_device
    }

    /// Returns a reference to the `AudioDeviceType` of this device.
    pub fn device_type(&self) -> &AudioDeviceType {
        &self.device
    }

    /// Sets the I/O port address for this device.
    ///
    /// # Arguments
    ///
    /// * `io` - The I/O port address (in hexadecimal, e.g., `0x220`).
    pub fn set_io(&mut self, io: u16) {
        self.io = Some(io);
    }

    /// Sets the DMA channel number for this device.
    ///
    /// # Arguments
    ///
    /// * `dma` - The DMA channel number (typically 0–7).
    pub fn set_dma_low(&mut self, dma: u8) {
        self.dma_low = Some(dma);
    }

    /// Set the high (16-bit) DMA channel number for this device.
    ///
    /// # Arguments
    /// * `dma` - The DMA channel number (typically 0-7).
    pub fn set_dma_high(&mut self, dma: u8) {
        self.dma_high = Some(dma);
    }

    /// Sets the IRQ line number for this device.
    ///
    /// # Arguments
    ///
    /// * `irq` - The IRQ line number (typically 0–7).
    pub fn set_irq_low(&mut self, irq: u8) {
        self.irq_low = Some(irq);
    }

    /// Sets the 16-bit IRQ line number for this device.
    ///
    /// # Arguments
    ///
    /// * `irq` - The IRQ line number (typically 8-15)
    pub fn set_irq_high(&mut self, irq: u8) {
        self.irq_high = Some(irq);
    }

    /// Convenience method to set all hardware resources for this device at once.
    ///
    /// # Arguments
    ///
    /// * `io` - The I/O port address.
    /// * `dma` - The DMA channel number.
    /// * `irq` - The IRQ line number.
    pub fn configure(&mut self, io: u16, dma_low: u8, dma_high: u8, irq_low: u8, irq_high: u8) {
        self.set_io(io);
        self.set_dma_low(dma_low);
        self.set_dma_high(dma_high);
        self.set_irq_low(irq_low);
        self.set_irq_high(irq_high);
    }

    /// Returns the configured I/O port address, if any.
    pub fn io(&self) -> Option<u16> {
        self.io
    }

    /// Returns the configured DMA channel number, if any.
    pub fn dma_low(&self) -> Option<u8> {
        self.dma_low
    }

    /// Returns the configured DMA channel (16-bit), if any.
    pub fn dma_high(&self) -> Option<u8> {
        self.dma_high
    }

    /// Returns the configured IRQ line number, if any.
    pub fn irq_low(&self) -> Option<u8> {
        self.irq_low
    }

    /// Returns the configured 16-bit IRQ line number, if any.
    pub fn irq_high(&self) -> Option<u8> {
        self.irq_high
    }
}

impl FromStr for AudioDevice {
    type Err = HwSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let devicetype = AudioDeviceType::from_str(s)?;
        return Ok(AudioDevice::new(devicetype));
    }
}
