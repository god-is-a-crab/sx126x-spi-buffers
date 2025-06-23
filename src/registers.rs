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
    fn bits(&self) -> u8 {
        self.0
    }
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LoraSyncWordLsb(pub u8);
impl const Register for LoraSyncWordLsb {
    const ADDRESS: u16 = 0x0741;
    fn bits(&self) -> u8 {
        self.0
    }
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RandomNumberGen0(pub u8);
impl const Register for RandomNumberGen0 {
    const ADDRESS: u16 = 0x0819;
    fn bits(&self) -> u8 {
        self.0
    }
    fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RxGain(pub u8);
impl const Register for RxGain {
    const ADDRESS: u16 = 0x08AC;
    fn bits(&self) -> u8 {
        self.0
    }
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
        let write_register: WriteRegister = WriteRegister::new(RxGain(0x96));
        assert_eq!(write_register.tx_buf, [0x0D, 0x08, 0xAC, 0x96]);
    }
}
