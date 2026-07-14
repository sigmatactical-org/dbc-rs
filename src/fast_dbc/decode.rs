//! Pre-computed decode structures for fast signal extraction.

use crate::{ByteOrder, Signal};

/// Pre-computed signal decode parameters.
///
/// Packed for cache efficiency (fits in a single cache line per signal).
#[derive(Clone, Copy)]
pub(crate) struct SignalDecode {
    /// Starting byte index in payload
    pub byte_start: u8,
    /// Bit offset within starting byte (0-7)
    pub bit_offset: u8,
    /// Signal length in bits (up to the DBC max of 512, so must be 16-bit)
    pub length: u16,
    /// Flags: bit 0 = unsigned, bit 1 = little_endian, bit 2 = identity_transform
    pub flags: u8,
    /// Scaling factor
    pub factor: f64,
    /// Offset value
    pub offset: f64,
}

impl SignalDecode {
    /// Signal is unsigned.
    pub const FLAG_UNSIGNED: u8 = 0b0001;
    /// Signal is little-endian (Intel).
    pub const FLAG_LITTLE_ENDIAN: u8 = 0b0010;
    /// Factor 1 / offset 0 — raw equals physical.
    pub const FLAG_IDENTITY: u8 = 0b0100;

    #[inline(always)]
    pub fn is_unsigned(self) -> bool {
        (self.flags & Self::FLAG_UNSIGNED) != 0
    }

    #[inline(always)]
    pub fn is_little_endian(self) -> bool {
        (self.flags & Self::FLAG_LITTLE_ENDIAN) != 0
    }

    #[inline(always)]
    pub fn is_identity(self) -> bool {
        (self.flags & Self::FLAG_IDENTITY) != 0
    }

    /// Create from a Signal reference.
    pub fn from_signal(signal: &Signal) -> Self {
        let start_bit = signal.start_bit() as usize;
        let length = signal.length() as usize;

        let mut flags = 0u8;
        if signal.is_unsigned() {
            flags |= Self::FLAG_UNSIGNED;
        }
        if signal.byte_order() == ByteOrder::LittleEndian {
            flags |= Self::FLAG_LITTLE_ENDIAN;
        }
        // Identity transform: factor=1.0, offset=0.0
        if signal.factor() == 1.0 && signal.offset() == 0.0 {
            flags |= Self::FLAG_IDENTITY;
        }

        Self {
            byte_start: (start_bit / 8) as u8,
            bit_offset: (start_bit % 8) as u8,
            length: length as u16,
            flags,
            factor: signal.factor(),
            offset: signal.offset(),
        }
    }
}

/// Pre-computed decode plan for a message.
pub(crate) struct DecodePlan {
    /// Message index in the original Dbc
    pub message_index: usize,
    /// Minimum bytes required to decode all signals
    pub min_bytes: u8,
    /// Pre-computed signal decode parameters
    pub signals: Vec<SignalDecode>,
}
