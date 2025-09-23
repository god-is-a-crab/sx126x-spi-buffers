//! Generate SPI buffers for Semtech SX126x SPI commands.
#![allow(clippy::new_without_default)]

use super::registers::Register;
use bitfield_struct::bitfield;
use core::marker::PhantomData;

/// A descriptor for an SPI transfer - contains TX and RX buffer pointers and transfer length.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpiDescriptor {
    pub tx_buf_ptr: *const u8,
    pub rx_buf_ptr: *const u8,
    pub transfer_length: u16,
}

/// # SetSleep command
/// Sets the device to sleep mode.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetSleep};
///
/// const SET_SLEEP: SetSleep = SetSleep::new(true);
/// assert_eq!(SET_SLEEP.tx_buf, [0x84, 0x04]);
/// assert_eq!(SET_SLEEP.rx_buf, [0, 0]);
/// assert_eq!(SET_SLEEP.descriptor().transfer_length, 2);
/// ``````
pub struct SetSleep {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl SetSleep {
    const OPCODE: u8 = 0x84;

    #[inline]
    pub const fn new(warm_start: bool) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                SleepConfig::new().with_warm_start(warm_start).into_bits(),
            ],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}
#[bitfield(u8, order = Msb)]
struct SleepConfig {
    #[bits(5)]
    __: u8,

    #[bits(1)]
    pub warm_start: bool,

    #[bits(2)]
    __: u8,
}

/// # SetStandby command
/// Sets the device to standby mode.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetStandby, StdbyConfig};
///
/// const SET_STANDBY: SetStandby = SetStandby::new(StdbyConfig::StdbyXosc);
/// assert_eq!(SET_STANDBY.tx_buf, [0x80, 1]);
/// assert_eq!(SET_STANDBY.rx_buf, [0, 0]);
/// assert_eq!(SET_STANDBY.descriptor().transfer_length, 2);
/// ```
pub struct SetStandby {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl SetStandby {
    const OPCODE: u8 = 0x80;

    #[inline]
    pub const fn new(stdby_config: StdbyConfig) -> Self {
        Self {
            tx_buf: [Self::OPCODE, stdby_config as u8],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}
#[repr(u8)]
pub enum StdbyConfig {
    StdbyRc = 0,
    StdbyXosc = 1,
}

/// # SetFs command
/// Sets the device to frequency synthesis mode.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetFs;
///
/// const SET_FS: SetFs = SetFs::new();
/// assert_eq!(SET_FS.tx_buf, [0xC1]);
/// assert_eq!(SET_FS.rx_buf, [0; 1]);
/// assert_eq!(SET_FS.descriptor().transfer_length, 1);
/// ```
pub struct SetFs {
    pub tx_buf: [u8; 1],
    pub rx_buf: [u8; 1],
}
impl SetFs {
    const OPCODE: u8 = 0xC1;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE],
            rx_buf: [0; 1],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 1,
        }
    }
}

/// # SetTx command
/// Sets the device to transmit mode with a specified timeout.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetTx;
///
/// const SET_TX: SetTx = SetTx::new(6862921);
/// assert_eq!(SET_TX.tx_buf, [0x83, 0x68, 0xB8, 0x49]);
/// assert_eq!(SET_TX.rx_buf, [0; 4]);
/// assert_eq!(SET_TX.descriptor().transfer_length, 4);
/// ```
pub struct SetTx {
    pub tx_buf: [u8; 4],
    pub rx_buf: [u8; 4],
}
impl SetTx {
    const OPCODE: u8 = 0x83;

    #[inline]
    pub const fn new(timeout: u32) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (timeout >> 16) as u8,
                (timeout >> 8) as u8,
                timeout as u8,
            ],
            rx_buf: [0; 4],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 4,
        }
    }
}

/// # SetRx command
/// Sets the device to receive mode with a specified timeout.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetRx;
///
/// const SET_RX: SetRx = SetRx::new(120);
/// assert_eq!(SET_RX.tx_buf, [0x82, 0, 0, 120]);
/// assert_eq!(SET_RX.rx_buf, [0; 4]);
/// assert_eq!(SET_RX.descriptor().transfer_length, 4);
/// ```
pub struct SetRx {
    pub tx_buf: [u8; 4],
    pub rx_buf: [u8; 4],
}
impl SetRx {
    const OPCODE: u8 = 0x82;

    #[inline]
    pub const fn new(timeout: u32) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (timeout >> 16) as u8,
                (timeout >> 8) as u8,
                timeout as u8,
            ],
            rx_buf: [0; 4],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 4,
        }
    }
}

/// # StopTimerOnPreamble command
/// Select if the timer stopped upon preamble detection or Sync Word/header detection.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::StopTimerOnPreamble;
///
/// const STOP_TIMER_ON_PREAMBLE: StopTimerOnPreamble = StopTimerOnPreamble::new(true);
/// assert_eq!(STOP_TIMER_ON_PREAMBLE.tx_buf, [0x9F, 1]);
/// assert_eq!(STOP_TIMER_ON_PREAMBLE.rx_buf, [0; 2]);
/// assert_eq!(STOP_TIMER_ON_PREAMBLE.descriptor().transfer_length, 2);
/// ```
pub struct StopTimerOnPreamble {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl StopTimerOnPreamble {
    const OPCODE: u8 = 0x9F;

    #[inline]
    pub const fn new(stop_on_preamble: bool) -> Self {
        Self {
            tx_buf: [Self::OPCODE, stop_on_preamble as u8],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}

/// # SetRxDutyCycle command
/// Sets the chip in sniff mode so that it regularly looks for new packets.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetRxDutyCycle;
///
/// const SET_RX_DUTY_CYCLE: SetRxDutyCycle = SetRxDutyCycle::new(1000, 4000);
/// assert_eq!(SET_RX_DUTY_CYCLE.tx_buf, [0x94, 0, 0x03, 0xE8, 0, 0x0F, 0xA0]);
/// assert_eq!(SET_RX_DUTY_CYCLE.rx_buf, [0; 7]);
/// assert_eq!(SET_RX_DUTY_CYCLE.descriptor().transfer_length, 7);
/// ```
pub struct SetRxDutyCycle {
    pub tx_buf: [u8; 7],
    pub rx_buf: [u8; 7],
}
impl SetRxDutyCycle {
    const OPCODE: u8 = 0x94;

    #[inline]
    pub const fn new(rx_period: u32, sleep_period: u32) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (rx_period >> 16) as u8,
                (rx_period >> 8) as u8,
                rx_period as u8,
                (sleep_period >> 16) as u8,
                (sleep_period >> 8) as u8,
                sleep_period as u8,
            ],
            rx_buf: [0; 7],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 7,
        }
    }
}

/// # SetCad command
/// Sets the device to Channel Activity Detection (CAD) mode.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetCad;
///
/// const SET_CAD: SetCad = SetCad::new();
/// assert_eq!(SET_CAD.tx_buf, [0xC5]);
/// assert_eq!(SET_CAD.rx_buf, [0; 1]);
/// assert_eq!(SET_CAD.descriptor().transfer_length, 1);
/// ```
pub struct SetCad {
    pub tx_buf: [u8; 1],
    pub rx_buf: [u8; 1],
}
impl SetCad {
    const OPCODE: u8 = 0xC5;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE],
            rx_buf: [0; 1],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 1,
        }
    }
}

/// # SetTxContinuousWave command
/// Test command to generate a continuous wave signal.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetTxContinuousWave;
///
/// const SET_TX_CONTINUOUS_WAVE: SetTxContinuousWave = SetTxContinuousWave::new();
/// assert_eq!(SET_TX_CONTINUOUS_WAVE.tx_buf, [0xD1]);
/// assert_eq!(SET_TX_CONTINUOUS_WAVE.rx_buf, [0; 1]);
/// assert_eq!(SET_TX_CONTINUOUS_WAVE.descriptor().transfer_length, 1);
/// ```
pub struct SetTxContinuousWave {
    pub tx_buf: [u8; 1],
    pub rx_buf: [u8; 1],
}
impl SetTxContinuousWave {
    const OPCODE: u8 = 0xD1;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE],
            rx_buf: [0; 1],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 1,
        }
    }
}

/// # SetTxInfinitePreamble command
/// Test command to generates an infinite sequence of alternating zeros and ones in FSK modulation.
/// In LoRa, the radio constantly modulates LoRa preamble symbols.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetTxInfinitePreamble;
///
/// const SET_TX_INFINITE_PREAMBLE: SetTxInfinitePreamble = SetTxInfinitePreamble::new();
/// assert_eq!(SET_TX_INFINITE_PREAMBLE.tx_buf, [0xD2]);
/// assert_eq!(SET_TX_INFINITE_PREAMBLE.rx_buf, [0; 1]);
/// assert_eq!(SET_TX_INFINITE_PREAMBLE.descriptor().transfer_length, 1);
/// ```
pub struct SetTxInfinitePreamble {
    pub tx_buf: [u8; 1],
    pub rx_buf: [u8; 1],
}
impl SetTxInfinitePreamble {
    const OPCODE: u8 = 0xD2;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE],
            rx_buf: [0; 1],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 1,
        }
    }
}

/// # SetRegulatorMode command
/// Configures the regulator mode (DC-DC or LDO).
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetRegulatorMode;
///
/// const SET_REGULATOR_MODE: SetRegulatorMode = SetRegulatorMode::new(true);
/// assert_eq!(SET_REGULATOR_MODE.tx_buf, [0x96, 1]);
/// assert_eq!(SET_REGULATOR_MODE.rx_buf, [0; 2]);
/// assert_eq!(SET_REGULATOR_MODE.descriptor().transfer_length, 2);
/// ```
pub struct SetRegulatorMode {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl SetRegulatorMode {
    const OPCODE: u8 = 0x96;

    #[inline]
    pub const fn new(dc_dc_mode: bool) -> Self {
        Self {
            tx_buf: [Self::OPCODE, dc_dc_mode as u8],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}

/// # Calibrate command
/// At power up the radio performs calibration of RC64k, RC13M, PLL and ADC.
/// It is however possible to launch a calibration of one or several blocks at any time starting in STDBY_RC mode.
/// The total calibration time if all blocks are calibrated is 3.5ms.
/// The calibration must be launched in STDBY_RC mode and the BUSY pins are high during the calibration process.
/// A falling edge of BUSY indicates the end of the procedure.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{Calibrate, CalibrationSetting};
///
/// const CALIBRATE: Calibrate = Calibrate::new(CalibrationSetting::new()
///     .with_rc64k(true)
///     .with_rc13m(true)
///     .with_pll(false)
///     .with_adc_pulse(false)
///     .with_adc_bulk_n(true)
///     .with_adc_bulk_p(false)
///     .with_image(true));
/// assert_eq!(CALIBRATE.tx_buf, [0x89, 0x53]);
/// assert_eq!(CALIBRATE.rx_buf, [0; 2]);
/// assert_eq!(CALIBRATE.descriptor().transfer_length, 2);
/// ```
pub struct Calibrate {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl Calibrate {
    const OPCODE: u8 = 0x89;

    #[inline]
    pub const fn new(calib_param: CalibrationSetting) -> Self {
        Self {
            tx_buf: [Self::OPCODE, calib_param.into_bits()],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}
/// Calibration settings for the Calibrate command. Select which blocks to calibrate.
#[bitfield(u8)]
pub struct CalibrationSetting {
    #[bits(1)]
    pub rc64k: bool,
    #[bits(1)]
    pub rc13m: bool,
    #[bits(1)]
    pub pll: bool,
    #[bits(1)]
    pub adc_pulse: bool,
    #[bits(1)]
    pub adc_bulk_n: bool,
    #[bits(1)]
    pub adc_bulk_p: bool,
    #[bits(1)]
    pub image: bool,
    #[bits(1)]
    __: bool,
}

/// # CalibrateImage command
/// Calibrate the image rejection of the device for the device operating frequency band.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::CalibrateImage;
///
/// const CALIBRATE_IMAGE: CalibrateImage = CalibrateImage::new(0xC4, 0x85);
/// assert_eq!(CALIBRATE_IMAGE.tx_buf, [0x98, 0xC4, 0x85]);
/// assert_eq!(CALIBRATE_IMAGE.rx_buf, [0; 3]);
/// assert_eq!(CALIBRATE_IMAGE.descriptor().transfer_length, 3);
/// ```
pub struct CalibrateImage {
    pub tx_buf: [u8; 3],
    pub rx_buf: [u8; 3],
}
impl CalibrateImage {
    const OPCODE: u8 = 0x98;

    #[inline]
    pub const fn new(freq1: u8, freq2: u8) -> Self {
        Self {
            tx_buf: [Self::OPCODE, freq1, freq2],
            rx_buf: [0; 3],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 3,
        }
    }
}

/// # SetPaConfig command
/// Configures the power amplifier settings.
///
/// - `device_sel = 1` only for SX1261.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetPaConfig;
///
/// const SET_PA_CONFIG: SetPaConfig = SetPaConfig::new(0x04, 0x07, 0);
/// assert_eq!(SET_PA_CONFIG.tx_buf, [0x95, 0x04, 0x07, 0x00, 0x01]);
/// assert_eq!(SET_PA_CONFIG.rx_buf, [0; 5]);
/// assert_eq!(SET_PA_CONFIG.descriptor().transfer_length, 5);
/// ```
pub struct SetPaConfig {
    pub tx_buf: [u8; 5],
    pub rx_buf: [u8; 5],
}
impl SetPaConfig {
    const OPCODE: u8 = 0x95;

    #[inline]
    pub const fn new(pa_duty_cycle: u8, hp_max: u8, device_sel: u8) -> Self {
        Self {
            tx_buf: [Self::OPCODE, pa_duty_cycle, hp_max, device_sel, 0x01],
            rx_buf: [0; 5],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 5,
        }
    }
}

/// # SetRxTxFallbackMode command
/// Defines into which mode the chip goes after a successful transmission or after a packet reception.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetRxTxFallbackMode, FallbackMode};
///
/// const SET_RX_TX_FALLBACK_MODE: SetRxTxFallbackMode = SetRxTxFallbackMode::new(FallbackMode::StdbyRc);
/// assert_eq!(SET_RX_TX_FALLBACK_MODE.tx_buf, [0x93, 0x20]);
/// assert_eq!(SET_RX_TX_FALLBACK_MODE.rx_buf, [0; 2]);
/// assert_eq!(SET_RX_TX_FALLBACK_MODE.descriptor().transfer_length, 2);
/// ```
pub struct SetRxTxFallbackMode {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl SetRxTxFallbackMode {
    const OPCODE: u8 = 0x93;

    #[inline]
    pub const fn new(fallback_mode: FallbackMode) -> Self {
        Self {
            tx_buf: [Self::OPCODE, fallback_mode as u8],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}
#[repr(u8)]
pub enum FallbackMode {
    Fs = 0x40,
    StdbyXosc = 0x30,
    StdbyRc = 0x20,
}

/// # WriteRegister command
/// Write a single register.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::{registers, commands::{WriteRegister}};
///
/// const WRITE_REGISTER: WriteRegister = WriteRegister::new(registers::LoraSyncWordMsb(0x48));
/// assert_eq!(WRITE_REGISTER.tx_buf, [0x0D, 0x07, 0x40, 0x48]);
/// assert_eq!(WRITE_REGISTER.rx_buf, [0; 4]);
/// assert_eq!(WRITE_REGISTER.descriptor().transfer_length, 4);
/// ```
pub struct WriteRegister {
    pub tx_buf: [u8; 4],
    pub rx_buf: [u8; 4],
}
impl WriteRegister {
    const OPCODE: u8 = 0x0D;

    #[inline]
    pub const fn new<R: const Register>(register: R) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (R::ADDRESS >> 8) as u8,
                R::ADDRESS as u8,
                register.bits(),
            ],
            rx_buf: [0; 4],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 4,
        }
    }
}

/// # WriteRegisters command
/// Write multiple registers in a single SPI transaction.
/// The address is auto-incremented.
///
/// ### Type Parameter `N`
/// `N = M + 3` where `M` is the number of registers to write
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::{registers, commands::{WriteRegisters}};
///
/// const WRITE_REGISTERS: WriteRegisters<5> = WriteRegisters::<5>::new::<registers::LoraSyncWordMsb>([0x67, 0x98]);
/// assert_eq!(WRITE_REGISTERS.tx_buf, [0x0D, 0x07, 0x40, 0x67, 0x98]);
/// assert_eq!(WRITE_REGISTERS.rx_buf, [0; 5]);
/// assert_eq!(WRITE_REGISTERS.descriptor().transfer_length, 5);
/// ```
pub struct WriteRegisters<const N: usize> {
    pub tx_buf: [u8; N],
    pub rx_buf: [u8; N],
}
impl<const N: usize> WriteRegisters<N> {
    const OPCODE: u8 = 0x0D;

    #[inline]
    pub const fn new<R: const Register>(data: [u8; N - 3]) -> Self {
        let mut tx_buf = [0; N];
        tx_buf[0] = Self::OPCODE;
        tx_buf[1] = (R::ADDRESS >> 8) as u8;
        tx_buf[2] = R::ADDRESS as u8;
        let mut i: usize = 0;
        while i < N - 3 {
            tx_buf[i + 3] = data[i];
            i += 1;
        }
        Self {
            tx_buf,
            rx_buf: [0; N],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: N as u16,
        }
    }
}

/// # ReadRegister command
/// Read a single register.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::{registers, commands::{ReadRegister}};
///
/// let mut read_register: ReadRegister<registers::LoraSyncWordLsb> = ReadRegister::new();
/// assert_eq!(read_register.tx_buf, [0x1D, 0x07, 0x41, 0, 0]);
/// assert_eq!(read_register.rx_buf, [0; 5]);
/// assert_eq!(read_register.descriptor().transfer_length, 5);
/// read_register.rx_buf[4] = 0x86;
/// assert_eq!(read_register.register(), registers::LoraSyncWordLsb(0x86));
/// ```
pub struct ReadRegister<R> {
    pub tx_buf: [u8; 5],
    pub rx_buf: [u8; 5],
    register: PhantomData<R>,
}
impl<R: const Register> ReadRegister<R> {
    const OPCODE: u8 = 0x1D;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (R::ADDRESS >> 8) as u8,
                R::ADDRESS as u8,
                0,
                0,
            ],
            rx_buf: [0; 5],
            register: PhantomData,
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 5,
        }
    }
    #[inline]
    pub const fn register(&self) -> R {
        R::from_bits(self.rx_buf[4])
    }
}

/// # ReadRegisters command
/// Read multiple registers in a single SPI transaction.
/// The address is auto-incremented.
///
/// ## Type Parameter `N`
/// `N = M + 4` where `M` is the number of registers to read
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::{registers, commands::{ReadRegisters}};
///
/// let read_registers: ReadRegisters<6> = ReadRegisters::<6>::new::<registers::LoraSyncWordMsb>();
/// assert_eq!(read_registers.tx_buf, [0x1D, 0x07, 0x40, 0, 0, 0]);
/// assert_eq!(read_registers.rx_buf, [0; 6]);
/// assert_eq!(read_registers.descriptor().transfer_length, 6);
/// ```
pub struct ReadRegisters<const N: usize> {
    pub tx_buf: [u8; N],
    pub rx_buf: [u8; N],
}
impl<const N: usize> ReadRegisters<N> {
    const OPCODE: u8 = 0x1D;

    #[inline]
    pub const fn new<R: const Register>() -> Self {
        let mut tx_buf = [0; N];
        tx_buf[0] = Self::OPCODE;
        tx_buf[1] = (R::ADDRESS >> 8) as u8;
        tx_buf[2] = R::ADDRESS as u8;
        Self {
            tx_buf,
            rx_buf: [0; N],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: N as u16,
        }
    }
}

/// # WriteBuffer command
/// Stores data payload to be transmitted. The address is auto-incremented;
/// when it exceeds 255 it is wrapped back to 0.
///
/// #### Type Parameter `N`
/// `N` = data length + 2
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::WriteBuffer;
///
/// let mut write_buffer: WriteBuffer<7> = WriteBuffer::new(0x10, [b'h', b'e', b'l', b'l', b'o'].into());
/// assert_eq!(write_buffer.tx_buf, [0x0E, 0x10, b'h', b'e', b'l', b'l', b'o']);
/// assert_eq!(write_buffer.rx_buf, [0; 7]);
/// assert_eq!(write_buffer.descriptor().transfer_length, 7);
/// ```
pub struct WriteBuffer<const N: usize> {
    pub tx_buf: [u8; N],
    pub rx_buf: [u8; N],
}
impl<const N: usize> WriteBuffer<N> {
    const OPCODE: u8 = 0x0E;

    #[inline]
    pub const fn new(offset: u8, data: [u8; N - 2]) -> Self {
        let mut tx_buf = [0; N];
        tx_buf[0] = Self::OPCODE;
        tx_buf[1] = offset;
        let mut i: usize = 0;
        while i < N - 2 {
            tx_buf[i + 2] = data[i];
            i += 1;
        }
        Self {
            tx_buf,
            rx_buf: [0; N],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: N as u16,
        }
    }
}

/// # ReadBuffer command
/// Reads bytes of payload received starting at offset.
///
/// #### Type Parameter `N`
/// `N` = data length + 3
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::ReadBuffer;
///
/// let mut read_buffer: ReadBuffer<8> = ReadBuffer::new(0x17);
/// assert_eq!(read_buffer.tx_buf, [0x1E, 0x17, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(read_buffer.rx_buf, [0; 8]);
/// assert_eq!(read_buffer.descriptor().transfer_length, 8);
/// read_buffer.rx_buf[3..8].copy_from_slice(&[b'h', b'e', b'l', b'l', b'o']);
/// assert_eq!(read_buffer.data(), &[b'h', b'e', b'l', b'l', b'o']);
/// ```
pub struct ReadBuffer<const N: usize> {
    pub tx_buf: [u8; N],
    pub rx_buf: [u8; N],
}
impl<const N: usize> ReadBuffer<N> {
    const OPCODE: u8 = 0x1E;

    #[inline]
    pub const fn new(offset: u8) -> Self {
        let mut tx_buf = [0; N];
        tx_buf[0] = Self::OPCODE;
        tx_buf[1] = offset;
        Self {
            tx_buf,
            rx_buf: [0; N],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: N as u16,
        }
    }
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.rx_buf[3..N]
    }
}

/// # SetDioIrqParams command
/// Sets the DIO IRQ parameters for the device.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetDioIrqParams, Irq};
///
/// const SET_DIO_IRQ_PARAMS: SetDioIrqParams = SetDioIrqParams::new(
///     Irq::new().with_tx_done(true),
///     Irq::new().with_rx_done(true),
///     Irq::new().with_timeout(true),
///     Irq::new()
/// );
/// assert_eq!(SET_DIO_IRQ_PARAMS.tx_buf, [0x08, 0, 1, 0, 2, 2, 0, 0, 0]);
/// assert_eq!(SET_DIO_IRQ_PARAMS.rx_buf, [0; 9]);
/// assert_eq!(SET_DIO_IRQ_PARAMS.descriptor().transfer_length, 9);
/// ```
pub struct SetDioIrqParams {
    pub tx_buf: [u8; 9],
    pub rx_buf: [u8; 9],
}
impl SetDioIrqParams {
    const OPCODE: u8 = 0x08;

    #[inline]
    pub const fn new(irq_mask: Irq, dio1_mask: Irq, dio2_mask: Irq, dio3_mask: Irq) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (irq_mask.into_bits() >> 8) as u8,
                irq_mask.into_bits() as u8,
                (dio1_mask.into_bits() >> 8) as u8,
                dio1_mask.into_bits() as u8,
                (dio2_mask.into_bits() >> 8) as u8,
                dio2_mask.into_bits() as u8,
                (dio3_mask.into_bits() >> 8) as u8,
                dio3_mask.into_bits() as u8,
            ],
            rx_buf: [0; 9],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 9,
        }
    }
}
#[bitfield(u16)]
#[derive(PartialEq, Eq)]
pub struct Irq {
    #[bits(1)]
    pub tx_done: bool,
    #[bits(1)]
    pub rx_done: bool,
    #[bits(1)]
    pub preamble_detected: bool,
    #[bits(1)]
    pub sync_word_valid: bool,
    #[bits(1)]
    pub header_valid: bool,
    #[bits(1)]
    pub header_err: bool,
    #[bits(1)]
    pub crc_err: bool,
    #[bits(1)]
    pub cad_done: bool,
    #[bits(1)]
    pub cad_detected: bool,
    #[bits(1)]
    pub timeout: bool,
    #[bits(4)]
    __: u8,
    #[bits(1)]
    pub lr_fhss_hop: bool,
    #[bits(1)]
    __: bool,
}

/// # GetIrqStatus command
/// Retrieves the value of the IRQ register.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{GetIrqStatus, Irq};
///
/// let mut get_irq_status: GetIrqStatus = GetIrqStatus::new();
/// assert_eq!(get_irq_status.tx_buf, [0x12, 0, 0, 0]);
/// assert_eq!(get_irq_status.rx_buf, [0; 4]);
/// assert_eq!(get_irq_status.descriptor().transfer_length, 4);
/// get_irq_status.rx_buf[3] = 0x03;
/// assert_eq!(get_irq_status.irq_status(), Irq::new().with_tx_done(true).with_rx_done(true).with_timeout(false));
/// ```
pub struct GetIrqStatus {
    pub tx_buf: [u8; 4],
    pub rx_buf: [u8; 4],
}
impl GetIrqStatus {
    const OPCODE: u8 = 0x12;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0, 0],
            rx_buf: [0; 4],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 4,
        }
    }
    #[inline]
    pub const fn irq_status(&self) -> Irq {
        Irq::from_bits((self.rx_buf[2] as u16) << 8 | (self.rx_buf[3] as u16))
    }
}

/// # ClearIrqStatus command
/// Clears an IRQ flag in the IRQ register.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{ClearIrqStatus, Irq};
///
/// const CLEAR_IRQ_STATUS: ClearIrqStatus = ClearIrqStatus::new(Irq::new()
///     .with_header_valid(true)
///     .with_timeout(true));
/// assert_eq!(CLEAR_IRQ_STATUS.tx_buf, [0x02, 2, 16]);
/// assert_eq!(CLEAR_IRQ_STATUS.rx_buf, [0; 3]);
/// assert_eq!(CLEAR_IRQ_STATUS.descriptor().transfer_length, 3);
/// ```
pub struct ClearIrqStatus {
    pub tx_buf: [u8; 3],
    pub rx_buf: [u8; 3],
}
impl ClearIrqStatus {
    const OPCODE: u8 = 0x02;

    #[inline]
    pub const fn new(clear_irq_param: Irq) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (clear_irq_param.into_bits() >> 8) as u8,
                clear_irq_param.into_bits() as u8,
            ],
            rx_buf: [0; 3],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 3,
        }
    }
}

/// # SetDio2AsRfSwitchCtrl command
/// Used to configure DIO2 so that it can be used to control an external RF switch.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetDio2AsRfSwitchCtrl;
///
/// const SET_DIO2_AS_RF_SWITCH_CTRL: SetDio2AsRfSwitchCtrl = SetDio2AsRfSwitchCtrl::new(true);
/// assert_eq!(SET_DIO2_AS_RF_SWITCH_CTRL.tx_buf, [0x9D, 1]);
/// assert_eq!(SET_DIO2_AS_RF_SWITCH_CTRL.rx_buf, [0; 2]);
/// assert_eq!(SET_DIO2_AS_RF_SWITCH_CTRL.descriptor().transfer_length, 2);
/// ```
pub struct SetDio2AsRfSwitchCtrl {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl SetDio2AsRfSwitchCtrl {
    const OPCODE: u8 = 0x9D;

    #[inline]
    pub const fn new(enable: bool) -> Self {
        Self {
            tx_buf: [Self::OPCODE, enable as u8],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}

/// # SetDio3AsTcxoCtrl command
/// Configures the chip for an external TCXO reference voltage controlled by DIO3.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetDio3AsTcxoCtrl, TcxoVoltage};
///
/// const SET_DIO3_AS_TCXO_CTRL: SetDio3AsTcxoCtrl = SetDio3AsTcxoCtrl::new(TcxoVoltage::V3_3, 350000);
/// assert_eq!(SET_DIO3_AS_TCXO_CTRL.tx_buf, [0x97, 7, 0x05, 0x57, 0x30]);
/// assert_eq!(SET_DIO3_AS_TCXO_CTRL.rx_buf, [0; 5]);
/// assert_eq!(SET_DIO3_AS_TCXO_CTRL.descriptor().transfer_length, 5);
/// ```
pub struct SetDio3AsTcxoCtrl {
    pub tx_buf: [u8; 5],
    pub rx_buf: [u8; 5],
}
impl SetDio3AsTcxoCtrl {
    const OPCODE: u8 = 0x97;

    #[inline]
    pub const fn new(tcxo_voltage: TcxoVoltage, delay: u32) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                tcxo_voltage as u8,
                (delay >> 16) as u8,
                (delay >> 8) as u8,
                delay as u8,
            ],
            rx_buf: [0; 5],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 5,
        }
    }
}
#[repr(u8)]
pub enum TcxoVoltage {
    V1_6 = 0x00,
    V1_7 = 0x01,
    V1_8 = 0x02,
    V2_2 = 0x03,
    V2_4 = 0x04,
    V2_7 = 0x05,
    V3_0 = 0x06,
    V3_3 = 0x07,
}

/// # SetRfFrequency command
/// Sets the RF frequency for the device.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetRfFrequency;
///
/// const SET_RF_FREQUENCY: SetRfFrequency = SetRfFrequency::new(455_081_984);
/// assert_eq!(SET_RF_FREQUENCY.tx_buf, [0x86, 0x1B, 0x20, 0, 0]);
/// assert_eq!(SET_RF_FREQUENCY.rx_buf, [0; 5]);
/// assert_eq!(SET_RF_FREQUENCY.descriptor().transfer_length, 5);
/// ```
pub struct SetRfFrequency {
    pub tx_buf: [u8; 5],
    pub rx_buf: [u8; 5],
}
impl SetRfFrequency {
    const OPCODE: u8 = 0x86;

    #[inline]
    pub const fn new(rf_freq: u32) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (rf_freq >> 24) as u8,
                (rf_freq >> 16) as u8,
                (rf_freq >> 8) as u8,
                rf_freq as u8,
            ],
            rx_buf: [0; 5],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 5,
        }
    }
}

/// # SetPacketType command
/// Sets the packet type for the device.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetPacketType, PacketType};
///
/// const SET_PACKET_TYPE: SetPacketType = SetPacketType::new(PacketType::Lora);
/// assert_eq!(SET_PACKET_TYPE.tx_buf, [0x8A, 0x01]);
/// assert_eq!(SET_PACKET_TYPE.rx_buf, [0; 2]);
/// assert_eq!(SET_PACKET_TYPE.descriptor().transfer_length, 2);
/// ```
pub struct SetPacketType {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl SetPacketType {
    const OPCODE: u8 = 0x8A;

    #[inline]
    pub const fn new(packet_type: PacketType) -> Self {
        Self {
            tx_buf: [Self::OPCODE, packet_type as u8],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum PacketType {
    Gfsk = 0x00,
    Lora = 0x01,
    Reserved = 0x02,
    LrFhss = 0x03,
}
impl PacketType {
    #[inline]
    const fn from(value: u8) -> Self {
        unsafe { core::mem::transmute(value & 0x03) }
    }
}

/// # GetPacketType command
/// Retrieves the current packet type of the device.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{GetPacketType, PacketType};
///
/// const GET_PACKET_TYPE: GetPacketType = GetPacketType::new();
/// assert_eq!(GET_PACKET_TYPE.tx_buf, [0x11, 0, 0]);
/// assert_eq!(GET_PACKET_TYPE.rx_buf, [0; 3]);
/// assert_eq!(GET_PACKET_TYPE.descriptor().transfer_length, 3);
/// assert_eq!(GET_PACKET_TYPE.packet_type(), PacketType::Gfsk);
/// ```
pub struct GetPacketType {
    pub tx_buf: [u8; 3],
    pub rx_buf: [u8; 3],
}
impl GetPacketType {
    const OPCODE: u8 = 0x11;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0],
            rx_buf: [0; 3],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 3,
        }
    }
    #[inline]
    pub const fn packet_type(&self) -> PacketType {
        PacketType::from(self.rx_buf[2])
    }
}

/// # SetTxParams command
/// Sets the TX output power and TX ramping time.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetTxParams, RampTime};
///
/// const SET_TX_PARAMS: SetTxParams = SetTxParams::new(22, RampTime::Ramp200U);
/// assert_eq!(SET_TX_PARAMS.tx_buf, [0x8E, 22, 4]);
/// assert_eq!(SET_TX_PARAMS.rx_buf, [0; 3]);
/// assert_eq!(SET_TX_PARAMS.descriptor().transfer_length, 3);
/// ```
pub struct SetTxParams {
    pub tx_buf: [u8; 3],
    pub rx_buf: [u8; 3],
}
impl SetTxParams {
    const OPCODE: u8 = 0x8E;

    #[inline]
    pub const fn new(power: u8, ramp_time: RampTime) -> Self {
        Self {
            tx_buf: [Self::OPCODE, power, ramp_time as u8],
            rx_buf: [0; 3],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 3,
        }
    }
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum RampTime {
    Ramp10U = 0x00,
    Ramp20U = 0x01,
    Ramp40U = 0x02,
    Ramp80U = 0x03,
    Ramp200U = 0x04,
    Ramp800U = 0x05,
    Ramp1700U = 0x06,
    Ramp3400U = 0x07,
}

/// # SetModulationParamsLora command
/// Configures the LoRa modulation parameters of the radio.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetModulationParamsLora, Sf, Bw, Cr};
///
/// const SET_MODULATION_PARAMS_LORA: SetModulationParamsLora = SetModulationParamsLora::new(
///    Sf::Sf10,
///    Bw::Bw125,
///    Cr::Cr4_5,
///    false,
/// );
/// assert_eq!(SET_MODULATION_PARAMS_LORA.tx_buf, [0x8B, 0x0A, 0x04, 0x01, 0]);
/// assert_eq!(SET_MODULATION_PARAMS_LORA.rx_buf, [0; 5]);
/// assert_eq!(SET_MODULATION_PARAMS_LORA.descriptor().transfer_length, 5);
/// ```
pub struct SetModulationParamsLora {
    pub tx_buf: [u8; 5],
    pub rx_buf: [u8; 5],
}
impl SetModulationParamsLora {
    const OPCODE: u8 = 0x8B;

    #[inline]
    pub const fn new(sf: Sf, bw: Bw, cr: Cr, low_data_rate_optimize: bool) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                sf as u8,
                bw as u8,
                cr as u8,
                low_data_rate_optimize as u8,
            ],
            rx_buf: [0; 5],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 5,
        }
    }
}
#[repr(u8)]
pub enum Sf {
    Sf5 = 0x05,
    Sf6 = 0x06,
    Sf7 = 0x07,
    Sf8 = 0x08,
    Sf9 = 0x09,
    Sf10 = 0x0A,
    Sf11 = 0x0B,
    Sf12 = 0x0C,
}
#[repr(u8)]
pub enum Bw {
    Bw7_8 = 0x00,
    Bw10_42 = 0x08,
    Bw15_63 = 0x01,
    Bw20_83 = 0x09,
    Bw31_25 = 0x02,
    Bw41_67 = 0x0A,
    Bw62_50 = 0x03,
    Bw125 = 0x04,
    Bw250 = 0x05,
    Bw500 = 0x06,
}
#[repr(u8)]
pub enum Cr {
    Cr4_5 = 0x01,
    Cr4_6 = 0x02,
    Cr4_7 = 0x03,
    Cr4_8 = 0x04,
    Cr4_5Li = 0x05,
    Cr4_6Li = 0x06,
    Cr4_8Li = 0x07,
}

/// # SetPacketParams command
/// Sets the parameters of the packet handling block.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetPacketParams, HeaderType, InvertIq};
///
/// const SET_PACKET_PARAMS: SetPacketParams = SetPacketParams::new(
///    8,
///    HeaderType::VariableLength,
///    14,
///    false,
///    InvertIq::Standard,
/// );
/// assert_eq!(SET_PACKET_PARAMS.tx_buf, [0x8C, 0, 8, 0, 14, 0, 0]);
/// assert_eq!(SET_PACKET_PARAMS.rx_buf, [0; 7]);
/// assert_eq!(SET_PACKET_PARAMS.descriptor().transfer_length, 7);
/// ```
pub struct SetPacketParams {
    pub tx_buf: [u8; 7],
    pub rx_buf: [u8; 7],
}
impl SetPacketParams {
    const OPCODE: u8 = 0x8C;

    #[inline]
    pub const fn new(
        preamble_length: u16,
        header_type: HeaderType,
        payload_length: u8,
        crc_type: bool,
        invert_iq: InvertIq,
    ) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                (preamble_length >> 8) as u8,
                preamble_length as u8,
                header_type as u8,
                payload_length,
                crc_type as u8,
                invert_iq as u8,
            ],
            rx_buf: [0; 7],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 7,
        }
    }
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum HeaderType {
    VariableLength = 0x00,
    FixedLength = 0x01,
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum InvertIq {
    Standard = 0x00,
    Inverted = 0x01,
}

/// # SetCadParams command
/// Sets the parameters for the Channel Activity Detection (CAD) operation.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{SetCadParams, CadSymbolNum, CadExitMode};
///
/// const SET_CAD_PARAMS: SetCadParams = SetCadParams::new(CadSymbolNum::CadOn4Symb, 23, 10, CadExitMode::CadOnly, 1200);
/// assert_eq!(SET_CAD_PARAMS.tx_buf, [0x88, 2, 23, 10, 0, 0, 0x04, 0xB0]);
/// assert_eq!(SET_CAD_PARAMS.rx_buf, [0; 8]);
/// assert_eq!(SET_CAD_PARAMS.descriptor().transfer_length, 8);
/// ```
pub struct SetCadParams {
    pub tx_buf: [u8; 8],
    pub rx_buf: [u8; 8],
}
impl SetCadParams {
    const OPCODE: u8 = 0x88;

    #[inline]
    pub const fn new(
        symbol_num: CadSymbolNum,
        det_peak: u8,
        det_min: u8,
        exit_mode: CadExitMode,
        timeout: u32,
    ) -> Self {
        Self {
            tx_buf: [
                Self::OPCODE,
                symbol_num as u8,
                det_peak,
                det_min,
                exit_mode as u8,
                (timeout >> 16) as u8,
                (timeout >> 8) as u8,
                timeout as u8,
            ],
            rx_buf: [0; 8],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 8,
        }
    }
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum CadSymbolNum {
    CadOn1Symb = 0x00,
    CadOn2Symb = 0x01,
    CadOn4Symb = 0x02,
    CadOn8Symb = 0x03,
    CadOn16Symb = 0x04,
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum CadExitMode {
    CadOnly = 0x00,
    CadRx = 0x01,
}

/// # SetBufferBaseAddress command
/// Sets the base addresses for the TX and RX buffers.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetBufferBaseAddress;
/// const SET_BUFFER_BASE_ADDRESS: SetBufferBaseAddress = SetBufferBaseAddress::new(0x00, 0x80);
/// assert_eq!(SET_BUFFER_BASE_ADDRESS.tx_buf, [0x8F, 0, 128]);
/// assert_eq!(SET_BUFFER_BASE_ADDRESS.rx_buf, [0; 3]);
/// assert_eq!(SET_BUFFER_BASE_ADDRESS.descriptor().transfer_length, 3);
/// ```
pub struct SetBufferBaseAddress {
    pub tx_buf: [u8; 3],
    pub rx_buf: [u8; 3],
}
impl SetBufferBaseAddress {
    const OPCODE: u8 = 0x8F;

    #[inline]
    pub const fn new(tx_base_address: u8, rx_base_address: u8) -> Self {
        Self {
            tx_buf: [Self::OPCODE, tx_base_address, rx_base_address],
            rx_buf: [0; 3],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 3,
        }
    }
}

/// # SetLoraSymbNumTimeout command
/// Sets the number of symbols used by the modem to validate a successful reception.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::SetLoraSymbNumTimeout;
///
/// const SET_LORA_SYMB_NUM_TIMEOUT: SetLoraSymbNumTimeout = SetLoraSymbNumTimeout::new(6);
/// assert_eq!(SET_LORA_SYMB_NUM_TIMEOUT.tx_buf, [0xA0, 6]);
/// assert_eq!(SET_LORA_SYMB_NUM_TIMEOUT.rx_buf, [0; 2]);
/// assert_eq!(SET_LORA_SYMB_NUM_TIMEOUT.descriptor().transfer_length, 2);
/// ```
pub struct SetLoraSymbNumTimeout {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl SetLoraSymbNumTimeout {
    const OPCODE: u8 = 0xA0;

    #[inline]
    pub const fn new(symb_num: u8) -> Self {
        Self {
            tx_buf: [Self::OPCODE, symb_num],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
}

/// # GetStatus command
/// Retrieves the current status of the device.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{GetStatus, Status, ChipMode, CommandStatus};
///
/// let mut get_status: GetStatus = GetStatus::new();
/// assert_eq!(get_status.tx_buf, [0xC0, 0]);
/// assert_eq!(get_status.rx_buf, [0; 2]);
/// assert_eq!(get_status.descriptor().transfer_length, 2);
///
/// get_status.rx_buf[1] = 0x64;
/// assert_eq!(get_status.status().chip_mode(), ChipMode::Tx);
/// assert_eq!(get_status.status().command_status(), CommandStatus::DataIsAvailableToHost);
/// ```
pub struct GetStatus {
    pub tx_buf: [u8; 2],
    pub rx_buf: [u8; 2],
}
impl GetStatus {
    const OPCODE: u8 = 0xC0;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0],
            rx_buf: [0; 2],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 2,
        }
    }
    #[inline]
    pub const fn status(&self) -> Status {
        Status::from_bits(self.rx_buf[1])
    }
}
#[bitfield(u8, order = Msb)]
#[derive(PartialEq, Eq)]
pub struct Status {
    #[bits(1)]
    __: bool,
    #[bits(3)]
    pub chip_mode: ChipMode,
    #[bits(3)]
    pub command_status: CommandStatus,
    #[bits(1)]
    __: bool,
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum ChipMode {
    Unused = 0x0,
    Reserved1 = 0x1,
    StbyRc = 0x2,
    StbyXosc = 0x3,
    Fs = 0x4,
    Rx = 0x5,
    Tx = 0x6,
    Reserved2 = 0x07,
}
impl ChipMode {
    #[inline]
    const fn into_bits(self) -> u8 {
        self as u8
    }
    #[inline]
    const fn from_bits(value: u8) -> Self {
        unsafe { core::mem::transmute(value & 0x07) }
    }
}
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum CommandStatus {
    Reserved1 = 0x0,
    Reserved2 = 0x1,
    DataIsAvailableToHost = 0x2,
    CommandTimeout = 0x3,
    CommandProcessingError = 0x4,
    FailureToExecuteCommand = 0x5,
    CommandTxDone = 0x6,
    Reserved3 = 0x07,
}
impl CommandStatus {
    #[inline]
    const fn into_bits(self) -> u8 {
        self as u8
    }
    #[inline]
    const fn from_bits(value: u8) -> Self {
        unsafe { core::mem::transmute(value & 0x07) }
    }
}

/// # GetRssiInst command
/// Gets the instantaneous RSSI value during reception of the packet.
/// This command is valid for all protocols.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::GetRssiInst;
///
/// let mut get_rssi_inst: GetRssiInst = GetRssiInst::new();
/// assert_eq!(get_rssi_inst.tx_buf, [0x15, 0, 0]);
/// assert_eq!(get_rssi_inst.rx_buf, [0; 3]);
/// assert_eq!(get_rssi_inst.descriptor().transfer_length, 3);
///
/// get_rssi_inst.rx_buf[2] = 44;
/// assert_eq!(get_rssi_inst.rssi_inst(), -22);
/// ```
pub struct GetRssiInst {
    pub tx_buf: [u8; 3],
    pub rx_buf: [u8; 3],
}
impl GetRssiInst {
    const OPCODE: u8 = 0x15;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0],
            rx_buf: [0; 3],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 3,
        }
    }
    #[inline]
    pub const fn rssi_inst(&self) -> i8 {
        -((self.rx_buf[2] / 2) as i8)
    }
}

/// # GetRxBufferStatus command
/// Returns the length of the last received packet (PayloadLengthRx) and
/// the address of the first byte received (RxStartBufferPointer).
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::GetRxBufferStatus;
///
/// let mut get_rx_buffer_status: GetRxBufferStatus = GetRxBufferStatus::new();
/// assert_eq!(get_rx_buffer_status.tx_buf, [0x13, 0, 0, 0]);
/// assert_eq!(get_rx_buffer_status.rx_buf, [0; 4]);
/// assert_eq!(get_rx_buffer_status.descriptor().transfer_length, 4);
///
/// get_rx_buffer_status.rx_buf[2] = 16;
/// get_rx_buffer_status.rx_buf[3] = 8;
/// assert_eq!(get_rx_buffer_status.payload_length_rx(), 16);
/// assert_eq!(get_rx_buffer_status.rx_start_buffer_pointer(), 8);
/// ```
pub struct GetRxBufferStatus {
    pub tx_buf: [u8; 4],
    pub rx_buf: [u8; 4],
}
impl GetRxBufferStatus {
    const OPCODE: u8 = 0x13;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0, 0],
            rx_buf: [0; 4],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 4,
        }
    }
    #[inline]
    pub const fn payload_length_rx(&self) -> u8 {
        self.rx_buf[2]
    }
    #[inline]
    pub const fn rx_start_buffer_pointer(&self) -> u8 {
        self.rx_buf[3]
    }
}

/// # GetPacketStatusLora command
/// Gets the signal quality of the last received LoRa packets.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::GetPacketStatusLora;
///
/// let mut get_packet_status_lora: GetPacketStatusLora = GetPacketStatusLora::new();
/// assert_eq!(get_packet_status_lora.tx_buf, [0x14, 0, 0, 0, 0]);
/// assert_eq!(get_packet_status_lora.rx_buf, [0; 5]);
/// assert_eq!(get_packet_status_lora.descriptor().transfer_length, 5);
///
/// get_packet_status_lora.rx_buf[2] = 184;
/// get_packet_status_lora.rx_buf[3] = 0b1111_1100;
/// get_packet_status_lora.rx_buf[4] = 162;
/// assert_eq!(get_packet_status_lora.rssi_pkt(), -92);
/// assert_eq!(get_packet_status_lora.snr_pkt(), -1);
/// assert_eq!(get_packet_status_lora.signal_rssi_pkt(), -81);
/// ```
pub struct GetPacketStatusLora {
    pub tx_buf: [u8; 5],
    pub rx_buf: [u8; 5],
}

impl GetPacketStatusLora {
    const OPCODE: u8 = 0x14;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0, 0, 0],
            rx_buf: [0; 5],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 5,
        }
    }
    #[inline]
    pub const fn rssi_pkt(&self) -> i8 {
        -((self.rx_buf[2] / 2) as i8)
    }
    #[inline]
    pub const fn snr_pkt(&self) -> i8 {
        (self.rx_buf[3] as i8) / 4
    }
    #[inline]
    pub const fn signal_rssi_pkt(&self) -> i8 {
        -((self.rx_buf[4] / 2) as i8)
    }
}

/// # GetStatsLora command
/// Returns the number of received packets, CRC errors, and header errors for LoRa packets.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::GetStatsLora;
///
/// let mut get_stats_lora: GetStatsLora = GetStatsLora::new();
/// assert_eq!(get_stats_lora.tx_buf, [0x10, 0, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(get_stats_lora.rx_buf, [0; 8]);
/// assert_eq!(get_stats_lora.descriptor().transfer_length, 8);
///
/// get_stats_lora.rx_buf[2] = 0x51;
/// get_stats_lora.rx_buf[3] = 0x18;
/// get_stats_lora.rx_buf[4] = 0x03;
/// get_stats_lora.rx_buf[5] = 0x15;
/// get_stats_lora.rx_buf[6] = 0x55;
/// get_stats_lora.rx_buf[7] = 0x81;
/// assert_eq!(get_stats_lora.nb_pkt_received(), 0x5118);
/// assert_eq!(get_stats_lora.nb_pkt_crc_error(), 0x0315);
/// assert_eq!(get_stats_lora.nb_pkt_header_err(), 0x5581);
/// ```
pub struct GetStatsLora {
    pub tx_buf: [u8; 8],
    pub rx_buf: [u8; 8],
}
impl GetStatsLora {
    const OPCODE: u8 = 0x10;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0, 0, 0, 0, 0, 0],
            rx_buf: [0; 8],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 8,
        }
    }
    #[inline]
    pub const fn nb_pkt_received(&self) -> u16 {
        (self.rx_buf[2] as u16) << 8 | (self.rx_buf[3]) as u16
    }
    #[inline]
    pub const fn nb_pkt_crc_error(&self) -> u16 {
        (self.rx_buf[4] as u16) << 8 | (self.rx_buf[5]) as u16
    }
    #[inline]
    pub const fn nb_pkt_header_err(&self) -> u16 {
        (self.rx_buf[6] as u16) << 8 | (self.rx_buf[7]) as u16
    }
}

/// # ResetStats command
/// Resets the number of packets received counters.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::ResetStats;
///
/// const RESET_STATS: ResetStats = ResetStats::new();
/// assert_eq!(RESET_STATS.tx_buf, [0x00, 0, 0, 0, 0, 0, 0]);
/// assert_eq!(RESET_STATS.rx_buf, [0; 7]);
/// assert_eq!(RESET_STATS.descriptor().transfer_length, 7);
/// ```
pub struct ResetStats {
    pub tx_buf: [u8; 7],
    pub rx_buf: [u8; 7],
}
impl ResetStats {
    const OPCODE: u8 = 0x00;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0, 0, 0, 0, 0],
            rx_buf: [0; 7],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 7,
        }
    }
}

/// # GetDeviceErrors command
/// Returns error flags.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::{GetDeviceErrors, OpError};
///
/// let mut get_device_errors: GetDeviceErrors = GetDeviceErrors::new();
/// assert_eq!(get_device_errors.tx_buf, [0x17, 0, 0, 0]);
/// assert_eq!(get_device_errors.rx_buf, [0; 4]);
/// assert_eq!(get_device_errors.descriptor().transfer_length, 4);
///
/// get_device_errors.rx_buf[2] = 0x01;
/// get_device_errors.rx_buf[3] = 0x58;
/// assert_eq!(get_device_errors.op_error(), OpError::new().with_pa_ramp_err(true)
///    .with_pll_lock_err(true).with_img_calib_err(true).with_adc_calib_err(true).with_xosc_start_err(false));
pub struct GetDeviceErrors {
    pub tx_buf: [u8; 4],
    pub rx_buf: [u8; 4],
}
impl GetDeviceErrors {
    const OPCODE: u8 = 0x17;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0, 0],
            rx_buf: [0; 4],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 4,
        }
    }
    #[inline]
    pub const fn op_error(&self) -> OpError {
        OpError::from_bits((self.rx_buf[2] as u16) << 8 | self.rx_buf[3] as u16)
    }
}
#[bitfield(u16)]
#[derive(PartialEq, Eq)]
pub struct OpError {
    #[bits(1)]
    pub rc64k_calib_err: bool,
    #[bits(1)]
    pub rc13m_calib_err: bool,
    #[bits(1)]
    pub pll_calib_err: bool,
    #[bits(1)]
    pub adc_calib_err: bool,
    #[bits(1)]
    pub img_calib_err: bool,
    #[bits(1)]
    pub xosc_start_err: bool,
    #[bits(1)]
    pub pll_lock_err: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub pa_ramp_err: bool,
    #[bits(7)]
    __: u8,
}

/// # ClearDeviceErrors command
/// Clears the error flags.
///
/// ## Example
/// ```
/// use sx126x_spi_buffers::commands::ClearDeviceErrors;
///
/// const CLEAR_DEVICE_ERRORS: ClearDeviceErrors = ClearDeviceErrors::new();
/// assert_eq!(CLEAR_DEVICE_ERRORS.tx_buf, [0x07, 0, 0]);
/// assert_eq!(CLEAR_DEVICE_ERRORS.rx_buf, [0; 3]);
/// assert_eq!(CLEAR_DEVICE_ERRORS.descriptor().transfer_length, 3);
/// ```
pub struct ClearDeviceErrors {
    pub tx_buf: [u8; 3],
    pub rx_buf: [u8; 3],
}
impl ClearDeviceErrors {
    const OPCODE: u8 = 0x07;

    #[inline]
    pub const fn new() -> Self {
        Self {
            tx_buf: [Self::OPCODE, 0, 0],
            rx_buf: [0; 3],
        }
    }
    #[inline]
    pub const fn descriptor(&self) -> SpiDescriptor {
        SpiDescriptor {
            tx_buf_ptr: self.tx_buf.as_ptr(),
            rx_buf_ptr: self.rx_buf.as_ptr(),
            transfer_length: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::LoraSyncWordMsb;

    #[test]
    fn test_set_standby_rc() {
        static SET_STANDBY_RC: SetStandby = SetStandby::new(StdbyConfig::StdbyRc);
        assert_eq!(SET_STANDBY_RC.tx_buf, [0x80, 0])
    }

    #[test]
    fn test_set_packet_type() {
        static SET_PACKET_TYPE: SetPacketType = SetPacketType::new(PacketType::Lora);
        assert_eq!(SET_PACKET_TYPE.tx_buf, [0x8A, 0x01]);
    }

    #[test]
    fn test_get_packet_params() {
        let mut get_packet_type: GetPacketType = GetPacketType::new();
        get_packet_type.rx_buf[2] = 0x03;
        assert_eq!(get_packet_type.packet_type(), PacketType::LrFhss);
        get_packet_type.rx_buf[2] = 0x05;
        assert_eq!(get_packet_type.packet_type(), PacketType::Lora);
    }

    #[test]
    fn test_set_rf_frequency() {
        static SET_RF_FREQUENCY: SetRfFrequency = SetRfFrequency::new(455_081_984);
        assert_eq!(SET_RF_FREQUENCY.tx_buf, [0x86, 0x1B, 0x20, 0, 0]);
    }

    #[test]
    fn test_set_buffer_base_address() {
        static SET_BUFFER_BASE_ADDRESS: SetBufferBaseAddress =
            SetBufferBaseAddress::new(0x00, 0x80);
        assert_eq!(SET_BUFFER_BASE_ADDRESS.tx_buf, [0x8F, 0, 0x80]);
    }

    #[test]
    fn test_set_mod_params() {
        static SET_MODULATION_PARAMS_LORA: SetModulationParamsLora =
            SetModulationParamsLora::new(Sf::Sf10, Bw::Bw125, Cr::Cr4_5, false);
        assert_eq!(
            SET_MODULATION_PARAMS_LORA.tx_buf,
            [0x8B, 0x0A, 0x04, 0x01, 0]
        );
    }

    #[test]
    fn test_set_packet_params() {
        static SET_PACKET_PARAMS: SetPacketParams =
            SetPacketParams::new(8, HeaderType::VariableLength, 3, false, InvertIq::Standard);
        assert_eq!(SET_PACKET_PARAMS.tx_buf, [0x8C, 0, 8, 0, 3, 0, 0]);
    }

    #[test]
    fn test_set_dio_irq_params() {
        static SET_DIO_IRQ_PARAMS: SetDioIrqParams = SetDioIrqParams::new(
            Irq::new().with_tx_done(true).with_rx_done(true),
            Irq::new().with_tx_done(true).with_rx_done(true),
            Irq::new(),
            Irq::new(),
        );
        assert_eq!(SET_DIO_IRQ_PARAMS.tx_buf, [0x08, 0, 3, 0, 3, 0, 0, 0, 0]);
    }

    #[test]
    fn test_write_sync_word() {
        static WRITE_SYNC_WORD: WriteRegister = WriteRegister::new(LoraSyncWordMsb(0x14));
        assert_eq!(WRITE_SYNC_WORD.tx_buf, [0x0D, 0x07, 0x40, 0x14]);
    }

    #[test]
    fn test_set_pa_config() {
        static SET_PA_CONFIG: SetPaConfig = SetPaConfig::new(0x04, 0x07, 0);
        assert_eq!(SET_PA_CONFIG.tx_buf, [0x95, 0x04, 0x07, 0x00, 0x01]);
    }

    #[test]
    fn test_set_tx_params() {
        static SET_TX_PARAMS: SetTxParams = SetTxParams::new(22, RampTime::Ramp200U);
        assert_eq!(SET_TX_PARAMS.tx_buf, [0x8E, 0x16, 4]);
    }

    #[test]
    fn test_set_dio2_rf_switch_ctrl() {
        static SET_DIO2_RF_SWITCH_CTRL: SetDio2AsRfSwitchCtrl = SetDio2AsRfSwitchCtrl::new(true);
        assert_eq!(SET_DIO2_RF_SWITCH_CTRL.tx_buf, [0x9D, 1]);
    }

    #[test]
    fn test_write_buffer() {
        static WRITE_BUFFER: WriteBuffer<5> = WriteBuffer::new(0x00, [0x00, 0x00, 0x00]);
        assert_eq!(WRITE_BUFFER.tx_buf, [0x0E, 0, 0, 0, 0]);
    }

    #[test]
    fn test_set_tx() {
        static SET_TX: SetTx = SetTx::new(0x00);
        assert_eq!(SET_TX.tx_buf, [0x83, 0, 0, 0]);
    }
}
