//! <div class="warning">
//! <strong>Requires Rust Nightly</strong>
//! </div>
//!
#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![doc = include_str!("../README.md")]

pub mod commands;
pub mod registers;

#[cfg(test)]
mod tests {
    use super::commands::{self, SetSleep, SetStandby, SpiDescriptor, StdbyConfig, WriteBuffer};
    use arraydeque::ArrayDeque;

    unsafe impl Sync for commands::SpiDescriptor {}

    #[test]
    fn test_queue() {
        let mut queue: ArrayDeque<&commands::SpiDescriptor, 8> = ArrayDeque::new();
        static SET_SLEEP_BUFS: SetSleep = commands::SetSleep::new(true);
        static SET_STANDY_BUFS: SetStandby = commands::SetStandby::new(StdbyConfig::StdbyRc);
        static WRITE_BUFFER_BUFS: WriteBuffer<7> = commands::WriteBuffer::new(0, [2, 4, 7, 9, 3]);

        static SET_SLEEP: SpiDescriptor = SET_SLEEP_BUFS.descriptor();
        static SET_STANDBY: SpiDescriptor = SET_STANDY_BUFS.descriptor();
        static WRITE_BUFFER: SpiDescriptor = WRITE_BUFFER_BUFS.descriptor();

        let _ = queue.push_back(&SET_SLEEP);
        let _ = queue.push_back(&SET_STANDBY);
        let _ = queue.push_back(&WRITE_BUFFER);

        let mut desc = queue.pop_front().unwrap();
        let mut tx_buf =
            unsafe { core::slice::from_raw_parts(desc.tx_buf_ptr, desc.transfer_length as usize) };
        assert_eq!(tx_buf, [0x84, 1 << 2]);

        desc = queue.pop_front().unwrap();
        tx_buf =
            unsafe { core::slice::from_raw_parts(desc.tx_buf_ptr, desc.transfer_length as usize) };
        assert_eq!(tx_buf, [0x80, 0]);

        desc = queue.pop_front().unwrap();
        tx_buf =
            unsafe { core::slice::from_raw_parts(desc.tx_buf_ptr, desc.transfer_length as usize) };
        assert_eq!(tx_buf, [0x0E, 0, 2, 4, 7, 9, 3]);
    }
}
