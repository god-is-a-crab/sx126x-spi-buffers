//! Register definitions

#[const_trait]
pub trait Register: Copy {
    const ADDRESS: u16;
    fn bits(&self) -> u8;
    fn from_bits(bits: u8) -> Self;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LoraSyncWordMsb(pub u8);
impl const Register for LoraSyncWordMsb {
    const ADDRESS: u16 = 0x0740;
    #[inline]
    fn bits(&self) -> u8 {
        self.0
    }
    #[inline]
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LoraSyncWordLsb(pub u8);
impl const Register for LoraSyncWordLsb {
    const ADDRESS: u16 = 0x0741;
    #[inline]
    fn bits(&self) -> u8 {
        self.0
    }
    #[inline]
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RandomNumberGen0(pub u8);
impl const Register for RandomNumberGen0 {
    const ADDRESS: u16 = 0x0819;
    #[inline]
    fn bits(&self) -> u8 {
        self.0
    }
    #[inline]
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RxGain(pub RxGainSetting);
impl const Register for RxGain {
    const ADDRESS: u16 = 0x08AC;
    #[inline]
    fn bits(&self) -> u8 {
        self.0 as u8
    }
    #[inline]
    fn from_bits(bits: u8) -> Self {
        Self(RxGainSetting::from(bits))
    }
}
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RxGainSetting {
    Unknown = 0x00,
    PowerSaving = 0x94,
    Boosted = 0x96,
}
impl RxGainSetting {
    #[inline]
    const fn from(value: u8) -> Self {
        match value {
            0x94 => RxGainSetting::PowerSaving,
            0x96 => RxGainSetting::Boosted,
            _ => RxGainSetting::Unknown,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RxGainRetention0(pub u8);
impl const Register for RxGainRetention0 {
    const ADDRESS: u16 = 0x029F;
    #[inline]
    fn bits(&self) -> u8 {
        self.0
    }
    #[inline]
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RxGainRetention1(pub u8);
impl const Register for RxGainRetention1 {
    const ADDRESS: u16 = 0x02A0;
    #[inline]
    fn bits(&self) -> u8 {
        self.0
    }
    #[inline]
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RxGainRetention2(pub u8);
impl const Register for RxGainRetention2 {
    const ADDRESS: u16 = 0x02A1;
    #[inline]
    fn bits(&self) -> u8 {
        self.0
    }
    #[inline]
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{ReadRegisters, WriteRegister, WriteRegisters};

    #[test]
    fn test_write_lora_sync_word() {
        let write_registers: WriteRegisters<5> =
            WriteRegisters::<5>::new::<LoraSyncWordMsb>([0x64, 0x54]);
        assert_eq!(write_registers.tx_buf, [0x0D, 0x07, 0x40, 0x64, 0x54]);
    }

    #[test]
    fn test_random_number_gen() {
        let read_registers: ReadRegisters<8> = ReadRegisters::<8>::new::<RandomNumberGen0>();
        assert_eq!(read_registers.tx_buf, [0x1D, 0x08, 0x19, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_write_rx_gain() {
        let write_register: WriteRegister = WriteRegister::new(RxGain(RxGainSetting::Boosted));
        assert_eq!(write_register.tx_buf, [0x0D, 0x08, 0xAC, 0x96]);
    }

    #[test]
    fn test_write_rx_gain_retention_0() {
        let write_register: WriteRegister = WriteRegister::new(RxGainRetention0(0x01));
        assert_eq!(write_register.tx_buf, [0x0D, 0x02, 0x9F, 0x01]);
    }

    #[test]
    fn test_write_rx_gain_retention_1_2() {
        let write_registers: WriteRegisters<5> =
            WriteRegisters::<5>::new::<RxGainRetention1>([0x08, 0xAC]);
        assert_eq!(write_registers.tx_buf, [0x0D, 0x02, 0xA0, 0x08, 0xAC]);
    }
}
