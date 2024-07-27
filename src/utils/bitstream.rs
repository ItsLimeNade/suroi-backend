use std::cmp::min;

use super::{
    decimal::DecimalSerializer,
    string_utils::{read_ascii_string, read_utf8_string, write_ascii_string, write_utf8_string},
};

#[derive(Clone, Debug)]
pub struct BitStream {
    internal: Box<[u8]>,
    byte_length: usize,
    endianness: Endianness,
    index: usize,
}

impl BitStream {
    /// Creates a new bitstream with the specified length in bytes
    #[inline(always)]
    pub fn new(length: usize) -> BitStream {
        BitStream {
            internal: vec![0; length].into_boxed_slice(),
            byte_length: length,
            endianness: Endianness::Little,
            index: 0,
        }
    }

    /// Sets the stream's index, in bits
    pub fn set_index(&mut self, index: usize) {
        assert!(
            index < self.byte_length * 8,
            "Cannot set index to out-of-bounds value {}",
            index
        );
        self.index = index;
    }

    /// Sets the stream's endianness
    pub fn set_endianness(&mut self, endianness: Endianness) {
        self.endianness = endianness;
    }
}

pub trait Stream {
    fn byte_length(&self) -> usize;
    fn get_index(&self) -> usize;
    fn get_endianness(&self) -> Endianness;
    fn bits_left(&self) -> usize;

    fn read_bits(&mut self, bits: usize) -> u32;
    fn read_bits_signed(&mut self, bits: usize) -> i32;
    fn write_bits<T: Into<i32>>(&mut self, value: T, bits: usize);
    fn write_bits_us<T: Into<u32>>(&mut self, value: T, bits: usize);
    fn slice(&self, start: isize, end: isize) -> BitStream;

    // boolean
    fn write_boolean(&mut self, value: bool) {
        self.write_bits_us(if value { 1u32 } else { 0u32 }, 1);
    }

    fn read_boolean(&mut self) -> bool {
        self.read_bits(1) == 1
    }

    fn write_int4<T: Into<i8>>(&mut self, value: T) {
        self.write_bits_us((Into::<i8>::into(value) & 0b1111) as u32, 4);
    }

    // int4
    fn read_int4(&mut self) -> i8 {
        self.read_bits_signed(4) as i8
    }

    fn write_uint4<T: Into<u8>>(&mut self, value: T) {
        self.write_bits_us((Into::<u8>::into(value) & 0b1111) as u32, 4);
    }

    fn read_uint4(&mut self) -> u8 {
        self.read_bits(4) as u8
    }

    // int8
    fn write_int8<T: Into<i8>>(&mut self, value: T) {
        self.write_bits_us(Into::<i8>::into(value) as u32 & 0b1111_1111, 8);
    }

    fn read_int8(&mut self) -> i8 {
        self.read_bits_signed(8) as i8
    }

    fn write_uint8<T: Into<u8>>(&mut self, value: T) {
        self.write_bits_us(Into::<u8>::into(value) as u32, 8);
    }

    fn read_uint8(&mut self) -> u8 {
        self.read_bits(8) as u8
    }

    // int16
    fn write_int16<T: Into<i16>>(&mut self, value: T) {
        self.write_bits_us(Into::<i16>::into(value) as u32 & 0xFFFF, 16);
    }

    fn read_int16(&mut self) -> i16 {
        self.read_bits_signed(16) as i16
    }

    fn write_uint16<T: Into<u16>>(&mut self, value: T) {
        self.write_bits_us(Into::<u16>::into(value) as u32, 16);
    }

    fn read_uint16(&mut self) -> u16 {
        self.read_bits(16) as u16
    }

    // int32
    fn write_int32<T: Into<i32>>(&mut self, value: T) {
        self.write_uint32(value.into() as u32);
    }

    fn read_int32(&mut self) -> i32 {
        self.read_uint32() as i32
    }

    fn write_uint32<T: Into<u32>>(&mut self, value: T) {
        self.write_bits_us(Into::<u32>::into(value), 32);
    }

    fn read_uint32(&mut self) -> u32 {
        self.read_bits(32)
    }

    // int64
    fn write_int64<T: Into<i64>>(&mut self, value: T) {
        self.write_uint64(value.into() as u64);
    }

    fn read_int64(&mut self) -> i64 {
        self.read_uint64() as i64
    }

    fn write_uint64<T: Into<u64>>(&mut self, value: T) {
        let into = Into::<u64>::into(value);
        self.write_bits_us((into & 0xFFFFFFFF) as u32, 32);
        self.write_bits_us((into >> 32) as u32, 32);
    }

    fn read_uint64(&mut self) -> u64 {
        self.read_bits(32) as u64 + ((self.read_bits(32) as u64) << 32)
    }

    // int128
    fn write_int128<T: Into<i128>>(&mut self, value: T) {
        self.write_uint128(value.into() as u128);
    }

    fn read_int128(&mut self) -> i128 {
        self.read_uint128() as i128
    }

    fn write_uint128<T: Into<u128>>(&mut self, value: T) {
        let into = Into::<u128>::into(value);
        // needless operations added for padding & visual clarity
        self.write_bits_us((into & 0xFFFFFFFF) as u32, 32);
        self.write_bits_us(((into >> 0x20) & 0xFFFFFFFF) as u32, 32);
        self.write_bits_us(((into >> 0x40) & 0xFFFFFFFF) as u32, 32);
        self.write_bits_us(((into >> 0x60) & 0xFFFFFFFF) as u32, 32);
    }

    fn read_uint128(&mut self) -> u128 {
        // needless operations added for padding & visual clarity
        (self.read_bits(32) as u128)
            + ((self.read_bits(32) as u128) << 0x20)
            + ((self.read_bits(32) as u128) << 0x40)
            + ((self.read_bits(32) as u128) << 0x60)
    }

    // floats
    // TODO find a way to reuse these serializers

    // quarter-precision signed
    fn write_float8<T: Into<f64>>(&mut self, value: T) {
        self.write_uint8(DecimalSerializer::new(8, 3).encode_ieee(value) as u8);
    }

    fn read_float8(&mut self) -> f32 {
        DecimalSerializer::new(8, 3).decode_ieee(self.read_uint8()) as f32
    }

    // quarter-precision unsigned
    fn write_ufloat8<T: Into<f64>>(&mut self, value: T) {
        self.write_uint8(DecimalSerializer::new_unsigned(8, 3).encode_ieee(value) as u8);
    }

    fn read_ufloat8(&mut self) -> f32 {
        DecimalSerializer::new_unsigned(8, 3).decode_ieee(self.read_uint8()) as f32
    }

    // half-precision signed
    fn write_float16<T: Into<f64>>(&mut self, value: T) {
        self.write_uint16(DecimalSerializer::new(16, 5).encode_ieee(value) as u16);
    }

    fn read_float16(&mut self) -> f32 {
        DecimalSerializer::new(16, 5).decode_ieee(self.read_uint16()) as f32
    }

    // half-precision unsigned
    fn write_ufloat16<T: Into<f64>>(&mut self, value: T) {
        self.write_uint16(DecimalSerializer::new_unsigned(16, 5).encode_ieee(value) as u16);
    }

    fn read_ufloat16(&mut self) -> f32 {
        DecimalSerializer::new_unsigned(16, 5).decode_ieee(self.read_uint16()) as f32
    }

    // single-precision signed
    fn write_float32<T: Into<f64>>(&mut self, value: T) {
        self.write_uint32(DecimalSerializer::new(32, 8).encode_ieee(value) as u32);
    }

    fn read_float32(&mut self) -> f32 {
        DecimalSerializer::new(32, 8).decode_ieee(self.read_uint32()) as f32
    }

    // single-precision unsigned
    fn write_ufloat32<T: Into<f64>>(&mut self, value: T) {
        self.write_uint32(DecimalSerializer::new_unsigned(32, 8).encode_ieee(value) as u32);
    }

    fn read_ufloat32(&mut self) -> f64 {
        DecimalSerializer::new_unsigned(32, 8).decode_ieee(self.read_uint32())
    }

    // double-precision signed
    fn write_float64<T: Into<f64>>(&mut self, value: T) {
        self.write_uint64(DecimalSerializer::new(64, 11).encode_ieee(value) as u64);
    }

    fn read_float64(&mut self) -> f64 {
        DecimalSerializer::new(64, 11).decode_ieee(self.read_uint64())
    }

    // double-precision unsigned
    fn write_ufloat64<T: Into<f64>>(&mut self, value: T) {
        self.write_uint64(DecimalSerializer::new_unsigned(64, 11).encode_ieee(value) as u64);
    }

    fn read_ufloat64(&mut self) -> f64 {
        DecimalSerializer::new_unsigned(64, 11).decode_ieee(self.read_uint64())
    }

    // string
    fn write_ascii_string(&mut self, string: &str, bytes: Option<usize>) {
        write_ascii_string(self, string, bytes);
    }

    fn read_ascii_string(&mut self, bytes: Option<usize>) -> String {
        read_ascii_string(self, bytes)
    }

    fn write_utf8_string(&mut self, string: &str, bytes: Option<usize>) {
        write_utf8_string(self, string, bytes);
    }

    fn read_utf8_string(&mut self, bytes: Option<usize>) -> String {
        read_utf8_string(self, bytes)
    }

    // bitstream
    fn write_bitstream(&mut self, stream: &mut BitStream, bits: Option<usize>) {
        let mut to_write = bits.unwrap_or_else(|| self.bits_left());

        while to_write > 0 {
            let chunk = min(to_write, 32);
            self.write_bits_us(stream.read_bits(chunk), chunk);
            to_write -= chunk;
        }
    }

    fn read_bitstream(&mut self, bits: usize) -> BitStream {
        self.slice(
            self.get_index() as isize,
            (self.get_index() + bits) as isize,
        )
    }
}

impl Stream for BitStream {
    /// Returns the size of this bitstream in bytes
    #[inline(always)]
    fn byte_length(&self) -> usize {
        self.byte_length
    }

    /// Returns the stream's index, in bits
    #[inline(always)]
    fn get_index(&self) -> usize {
        self.index
    }

    /// Returns the stream's endianness
    #[inline(always)]
    fn get_endianness(&self) -> Endianness {
        self.endianness
    }

    #[inline(always)]
    fn bits_left(&self) -> usize {
        self.byte_length * 8 - self.index
    }

    /// Reads *up to 32 bits* from the underlying source, returning the result as an unsigned 32-bit integer
    fn read_bits(&mut self, bits: usize) -> u32 {
        assert!(bits <= 32, "Reads must be in chunks of at most 32 bits");

        let available = self.byte_length * 8 - self.index;
        if bits > available {
            panic!(
                "Cannot get {} bits from offset {}, {} available",
                bits, self.index, available
            );
        }

        let mut value: u32 = 0;
        let mut i = 0;

        while i < bits {
            let remaining = bits - i;
            let bit_offset = self.index & 7;
            let current_byte = self.internal.get(self.index >> 3).unwrap();

            // how many bits can be read from the current byte
            let to_read = min(remaining, 8 - bit_offset);
            let mask = !(0xFF << to_read);

            match self.endianness {
                Endianness::Big => {
                    let read_bits = ((current_byte >> (8 - to_read - bit_offset)) as u32) & mask;
                    value = value << to_read | read_bits;
                }
                Endianness::Little => {
                    let read_bits = ((current_byte >> bit_offset) as u32) & mask;
                    value |= read_bits << i;
                }
            }

            self.index += to_read;
            i += to_read;
        }

        value
    }

    /// Reads *up to 32 bits* from the underlying source, returning the result as a signed 32-bit integer
    fn read_bits_signed(&mut self, bits: usize) -> i32 {
        let mut value = self.read_bits(bits);
        /*
            If not working with a full 32 bits, check the
            imaginary MSB (most significant bit) for this bit
            count and convert to a valid 32-bit signed value,
            if set.

            For example, when working with 32 bits, 1111 is 15,
            but when working with 4 bits, it's -8.
        */
        let most_significant_bit = (1 << bits) - 1;
        if bits != 32 && (value & most_significant_bit) != 0 {
            value |= u32::MAX ^ most_significant_bit;
        }

        value as i32
    }

    /// Writes *up to 32 bits* to the underlying source.
    fn write_bits<T: Into<i32>>(&mut self, value: T, bits: usize) {
        self.write_bits_us(Into::<i32>::into(value) as u32, bits);
    }

    /// Writes *up to 32 bits* to the underlying source
    /// Provided for convenience when using unsigned integer types
    fn write_bits_us<T: Into<u32>>(&mut self, value: T, bits: usize) {
        assert!(bits <= 32, "Writes must be in chunks of at most 32 bits");

        let available = self.byte_length * 8 - self.index;
        if bits > available {
            panic!(
                "Cannot set {} bits from offset {}, {} available",
                bits, self.index, available
            );
        }

        let mut val: u32 = value.into();
        let mut i = 0;
        while i < bits {
            let remaining = bits - i;
            let bit_offset = self.index & 7;
            let byte_offset = self.index >> 3;

            // how many bits can be written to the current byte
            let to_write = min(remaining, 8 - bit_offset);

            match self.endianness {
                Endianness::Big => {
                    // create a mask with the correct width
                    let mask = !(!0 << to_write);
                    // shift the bits wanted to the start of the byte and mask off the rest
                    let write_bits = (val >> (bits - self.index - to_write)) & mask;
                    let dest_shift = 8 - bit_offset - to_write;
                    // Destination mask to zero all the bits being changed first
                    let dest_mask = !(mask << dest_shift);

                    let target = self.internal.get_mut(byte_offset).unwrap();
                    *target = ((*target as u32 & dest_mask) | (write_bits << dest_shift)) as u8;
                }
                Endianness::Little => {
                    // create a mask with the correct width
                    let mask = !(0xFF << to_write);
                    // shift the bits wanted to the start of the byte and mask off the rest
                    let write_bits = val & mask;
                    val >>= to_write;
                    // Destination mask to zero all the bits being changed first
                    let dest_mask = !(mask << bit_offset);

                    let target = self.internal.get_mut(byte_offset).unwrap();
                    *target = ((*target as u32 & dest_mask) | (write_bits << bit_offset)) as u8;
                }
            }

            self.index += to_write;
            i += to_write;
        }
    }

    /// Creates a new ArrayBuffer object whose contents are a copy of this instance's
    /// contents from `start` (inclusive) to `end` (exclusive). Negative indexes count
    /// backwards from the end of this instance's contents
    fn slice(&self, start: isize, end: isize) -> BitStream {
        let norm_start = if start.is_negative() {
            self.byte_length as isize
        } else {
            0
        } + start;

        let norm_end = if end.is_negative() {
            self.byte_length as isize
        } else {
            0
        } + end;

        assert!(norm_start > norm_end, "Start larger than end");

        let u_start = norm_start as usize;
        let u_end = norm_end as usize;

        assert!(
            u_start > self.byte_length,
            "Start index exceeds buffer length"
        );
        assert!(u_end > self.byte_length, "End index exceeds buffer length");

        BitStream {
            internal: self.internal[u_start..u_end].into(),
            byte_length: u_end - u_start,
            endianness: self.endianness,
            index: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Endianness {
    #[default]
    Little,
    Big,
}
