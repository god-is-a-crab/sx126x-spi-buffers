# SX126x SPI Buffers

Generate buffers and SPI byte sequences used for Semtech transceiver ICs:
- SX1268
- SX1261/2
- LLCC68

## Examples
### Create `SetPacketParams` command
```rust
use sx126x_spi_buffers::commands::{SetPacketParams, HeaderType, InvertIq};

const SET_PACKET_PARAMS: SetPacketParams = SetPacketParams::new(
   8,
   HeaderType::VariableLength,
   14,
   false,
   InvertIq::Standard,
);

assert_eq!(SET_PACKET_PARAMS.tx_buf, [0x8C, 0, 8, 0, 14, 0, 0]);
assert_eq!(SET_PACKET_PARAMS.rx_buf, [0; 7]);
assert_eq!(SET_PACKET_PARAMS.descriptor().transfer_length, 7);
```

### Write a register
```rust
use sx126x_spi_buffers::{commands::WriteRegister, registers::LoraSyncWordMsb};

const WRITE_SYNC_WORD: WriteRegister = WriteRegister::new(LoraSyncWordMsb(0x14));
assert_eq!(WRITE_SYNC_WORD.tx_buf, [0x0D, 0x07, 0x40, 0x14]);
```

### Write buffer
```rust
use sx126x_spi_buffers::commands::WriteBuffer;

static WRITE_BUFFER: WriteBuffer<9> = WriteBuffer::<9>::new(0x00,
    [b's', b'e', b'm', b't', b'e', b'c', b'h']);
assert_eq!(WRITE_BUFFER.tx_buf, [0x0E, 0, b's', b'e', b'm', b't', b'e', b'c', b'h']);
```
