use serde::Deserialize;
use std::str::FromStr;

use crate::error::HwSpecError;

/// Represents a specific type of audio device typically found in MS-DOS-compatible PC systems
/// manufactured between 1980 and 1996.
///
/// This enum provides a type-safe way to handle device identification and configuration.
///
/// # Examples
#[derive(Debug, Deserialize, PartialEq)]
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

impl FromStr for AudioDeviceType {
    type Err = HwSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AudioDeviceType::*;
        match s.trim().to_lowercase().as_str() {
            "bleeper" | "speaker" | "pcspeaker" | "pc speaker" => Ok(Bleeper),
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
            _ => Err(HwSpecError::InvalidAudioDevice),
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
    pub fn new(device: AudioDeviceType) -> Self {
        Self {
            device,
            io: None,
            dma: None,
            irq: None,
        }
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
    pub fn set_dma(&mut self, dma: u8) {
        self.dma = Some(dma);
    }

    /// Sets the IRQ line number for this device.
    ///
    /// # Arguments
    ///
    /// * `irq` - The IRQ line number (typically 0–15).
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
    pub fn configure(&mut self, io: u16, dma: u8, irq: u8) {
        self.set_io(io);
        self.set_dma(dma);
        self.set_irq(irq);
    }

    /// Returns the configured I/O port address, if any.
    pub fn io(&self) -> Option<u16> {
        self.io
    }

    /// Returns the configured DMA channel number, if any.
    pub fn dma(&self) -> Option<u8> {
        self.dma
    }

    /// Returns the configured IRQ line number, if any.
    pub fn irq(&self) -> Option<u8> {
        self.irq
    }
}

impl FromStr for AudioDevice {
    type Err = HwSpecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let devicetype = AudioDeviceType::from_str(s)?;
        return Ok(Self {
            device: devicetype,
            io: None,
            dma: None,
            irq: None,
        });
    }
}
