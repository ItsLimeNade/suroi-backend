use std::f64::consts::PI;

use crate::constants::{ObjectCategory, GAME_CONSTANTS};

use super::bitstream::{BitStream, Endianness, Stream};
use super::vectors::Vec2D;

use strum::EnumCount;

// FIXME
// pub const OBJECT_CATEGORY_BITS: usize = (ObjectCategory::COUNT as f64).log2().ceil() as usize;
pub const OBJECT_CATEGORY_BITS: usize = 4;
pub const OBJECT_ID_BITS: usize = 13;
pub const MIN_OBJECT_SCALE: f64 = 0.25;
pub const MAX_OBJECT_SCALE: f64 = 3.0;
pub const VARIATION_BITS: usize = 3;

#[derive(Clone, Debug)]
pub struct SuroiBitStream {
    internal: BitStream,
}

impl SuroiBitStream {
    #[inline(always)]
    pub fn new(bytes: usize) -> SuroiBitStream {
        SuroiBitStream {
            internal: BitStream::new(bytes),
        }
    }
}

// forwarded methods
impl Stream for SuroiBitStream {
    fn byte_length(&self) -> usize {
        self.internal.byte_length()
    }

    fn get_index(&self) -> usize {
        self.internal.get_index()
    }

    fn get_endianness(&self) -> Endianness {
        self.internal.get_endianness()
    }

    fn bits_left(&self) -> usize {
        self.internal.bits_left()
    }

    fn read_bits(&mut self, bits: usize) -> u32 {
        self.internal.read_bits(bits)
    }

    fn read_bits_signed(&mut self, bits: usize) -> i32 {
        self.internal.read_bits_signed(bits)
    }

    fn write_bits<T: Into<i32>>(&mut self, value: T, bits: usize) {
        self.internal.write_bits(value, bits)
    }

    fn write_bits_us<T: Into<u32>>(&mut self, value: T, bits: usize) {
        self.internal.write_bits_us(value, bits)
    }

    fn slice(&self, start: isize, end: isize) -> BitStream {
        self.internal.slice(start, end)
    }
}

impl SuroiBitStream {
    pub fn write_float(&mut self, value: f64, min: f64, max: f64, bit_count: usize) {
        self.write_bits_us(
            ((Into::<f64>::into(value).clamp(min, max) - min) / (max - min)
                * (((1u128 << bit_count) - 1) as f64)
                + 0.5)
                .trunc() as u32,
            bit_count,
        );
    }

    pub fn read_float(&mut self, min: f64, max: f64, bit_count: usize) -> f64 {
        min + (max - min) * (self.read_bits(bit_count) as f64) / ((1u128 << bit_count) - 1) as f64
    }

    pub fn write_vector(
        &mut self,
        vec: Vec2D,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        bit_count: usize,
    ) {
        self.write_float(
            Into::<Vec2D>::into(vec).x,
            min_x,
            max_x,
            bit_count,
        );
        self.write_float(
            Into::<Vec2D>::into(vec).y,
            min_y,
            max_y,
            bit_count,
        );
    }

    pub fn read_vector(
        &mut self,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        bit_count: usize,
    ) -> Vec2D {
        Vec2D::new(
            self.read_float(min_x, max_x, bit_count),
            self.read_float(min_y, max_y, bit_count),
        )
    }

    // FIXME
    // pub fn write_object_type(&mut self, object_type: ObjectCategory) {}
    // pub fn read_object_type(&mut self) -> ObjectCategory {}

    pub fn write_object_id(&mut self, id: u32) {
        self.write_bits_us(id, OBJECT_ID_BITS);
    }

    pub fn read_object_id(&mut self) -> u32 {
        self.read_bits(OBJECT_ID_BITS)
    }

    pub fn write_position(&mut self, vec: Vec2D) {
        self.write_vector(
            vec,
            0.0,
            0.0,
            GAME_CONSTANTS.max_position as f64,
            GAME_CONSTANTS.max_position as f64,
            16,
        );
    }

    pub fn read_position(&mut self) -> Vec2D {
        self.read_vector(
            0.0,
            0.0,
            GAME_CONSTANTS.max_position as f64,
            GAME_CONSTANTS.max_position as f64,
            16,
        )
    }

    pub fn write_rotation(&mut self, angle: f64, bit_count: usize) {
        self.write_float(angle, -PI, PI, bit_count);
    }

    pub fn read_rotation(&mut self, bit_count: usize) {
        self.read_float(-PI, PI, bit_count);
    }

    // FIXME
    // pub fn write_obstacle_rotation(&mut self, value: Into<f64>, mode: RotationMode) {}
    // pub fn read_obstacle_rotation(&mut self, mode: RotationMode) -> f64 {}

    pub fn write_scale(&mut self, angle: f64, bit_count: usize) {
        self.write_float(angle, MIN_OBJECT_SCALE, MAX_OBJECT_SCALE, bit_count);
    }

    pub fn read_scale(&mut self, bit_count: usize) {
        self.read_float(MIN_OBJECT_SCALE, MAX_OBJECT_SCALE, bit_count);
    }

    pub fn write_variation(&mut self, variation: u8) {
        self.write_bits_us(variation, VARIATION_BITS);
    }

    pub fn read_variation(&mut self) -> u8 {
        self.read_bits(VARIATION_BITS) as u8
    }

    pub fn write_player_name(&mut self, name: &str) {
        self.write_ascii_string(name, Some(GAME_CONSTANTS.player.name_max_length as usize));
    }

    pub fn read_player_name(&mut self) -> String {
        self.read_ascii_string(Some(GAME_CONSTANTS.player.name_max_length as usize))
    }

    pub fn write_array<T>(
        &mut self,
        arr: &Vec<T>,
        bit_count: usize,
        element_serializer: impl Fn(&T),
    ) {
        let length = arr.len();
        let max = 1u128 << bit_count;
        self.write_bits_us(length as u32, bit_count);

        for i in 0..length as u128 {
            if i > max {
                println!(
                    "writeArray: iterator overflow ({} bits, length {})",
                    bit_count, length
                );
                break;
            }

            element_serializer(&arr[i as usize]);
        }
    }

    pub fn read_array<'a, T>(
        &'a mut self,
        target: &'a mut Vec<T>,
        bit_count: usize,
        element_deserializer: impl Fn() -> T,
    ) -> &mut Vec<T> {
        for i in 0..self.read_bits(bit_count) {
            target.push(element_deserializer());
        }

        target
    }

    pub fn read_and_create_array<T>(
        &mut self,
        bit_count: usize,
        element_deserializer: impl Fn() -> T,
    ) -> Vec<T> {
        let length = self.read_bits(bit_count) as usize;
        let mut out: Vec<T> = Vec::with_capacity(length);

        for i in 0..length {
            out.push(element_deserializer());
        }

        out
    }

    // writeBytes can't easily be ported cuz no prive field access

    pub fn write_align_to_next_byte(&mut self) {
        let offset = 8 - self.get_index() % 8;
        if offset < 8 {
            self.write_bits(0, offset);
        }
    }

    pub fn read_align_to_next_byte(&mut self) {
        let offset = 8 - self.get_index() % 8;
        if offset < 8 {
            self.read_bits(offset);
        }
    }
}
