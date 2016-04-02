
// 21.21.21-bit morton codes: 63 bits total, largest that can fit in uint64_t
// 10.10.10-bit morton codes: 30 bits total, largest that can fit in uint32_t
// 7.7.7-bit morton codes: 21 bits total, components can be computed simultaneously in uint64_t
// 5.5.5-bit morton codes: 15 bits total, largest that can fit in uint16_t

#![feature(zero_one)]
extern crate num;

use std::ops::{Add, Sub, BitAnd, BitOr, Not};
use std::cmp::PartialOrd;



pub const MORTON3_21_X: u64 = 0x1249249249249249;
pub const MORTON3_21_Y: u64 = MORTON3_21_X << 1;
pub const MORTON3_21_Z: u64 = MORTON3_21_X << 2;
pub const MORTON3_21_MASK: u64 = MORTON3_21_X | MORTON3_21_Y | MORTON3_21_Z;

pub const MORTON3_7_X: u32 = 0x00049249;
pub const MORTON3_7_Y: u32 = MORTON3_7_X << 1;
pub const MORTON3_7_Z: u32 = MORTON3_7_X << 2;
pub const MORTON3_7_MASK: u32 = MORTON3_7_X | MORTON3_7_Y | MORTON3_7_Z;

pub const MORTON3_5_X: u16 = 0x1249;
pub const MORTON3_5_Y: u16 = MORTON3_5_X << 1;
pub const MORTON3_5_Z: u16 = MORTON3_5_X << 2;
pub const MORTON3_5_MASK: u16 = MORTON3_5_X | MORTON3_5_Y | MORTON3_5_Z;


pub fn spread2_16(x0: u32) -> u32 {
    //              0b00000000_00000000_11111111_11111111
    let x1 = ((x0 & 0b00000000_00000000_11111111_00000000) << 8) | (x0 & 0b00000000_00000000_00000000_11111111);
    //              0b00000000_11111111_00000000_11111111
    let x2 = ((x1 & 0b00000000_11110000_00000000_11110000) << 4) | (x1 & 0b00000000_00001111_00000000_00001111);
    //              0b00001111_00001111_00001111_00001111
    let x3 = ((x2 & 0b00001100_00001100_00001100_00001100) << 2) | (x2 & 0b00000011_00000011_00000011_00000011);
    //              0b00110011_00110011_00110011_00110011
             ((x3 & 0b00100010_00100010_00100010_00100010) << 1) | (x3 & 0b00010001_00010001_00010001_00010001)
    //              0b01010101_01010101_01010101_01010101
}


pub fn unspread2_16(x0: u32) -> u32 {
    //              0b01010101_01010101_01010101_01010101
    let x1 = ((x0 & 0b01000100_01000100_01000100_01000100) >> 1) | (x0 & 0b00010001_00010001_00010001_00010001);
    //              0b00110011_00110011_00110011_00110011
    let x2 = ((x1 & 0b00110000_00110000_00110000_00110000) >> 2) | (x1 & 0b00000011_00000011_00000011_00000011);
    //              0b00001111_00001111_00001111_00001111
    let x3 = ((x2 & 0b00001111_00000000_00001111_00000000) >> 4) | (x2 & 0b00000000_00001111_00000000_00001111);
    //              0b00000000_11111111_00000000_11111111
             ((x3 & 0b00000000_11111111_00000000_00000000) >> 8) | (x3 & 0b00000000_00000000_00000000_11111111)
    //              0b00000000_00000000_11111111_11111111
}


pub fn spread3_21(x: u32) -> u64 {
    // 0b0000000000000000000000000000000000000000000111111111111111111111
    // 0b0000000000000000000000011111111111000000000000000000001111111111
    // 0b0000000000000111111000000000011111000000000011111000000000011111
    // 0b0000000111000000111000011000000111000011000000111000011000000111
    // 0b0001000011001000011000011001000011000011001000011000011001000011
    // 0b0001001001001001001001001001001001001001001001001001001001001001
    let x0 = x as u64;
    let x1 = ((x0 & 0x00000000001FFC00) << 20) | (x0 & 0x00000000000003FF);
    let x2 = ((x1 & 0x000001F8000003E0) << 10) | (x1 & 0x00000007C000001F);
    let x3 = ((x2 & 0x00070006000C0018) << 6)  | (x2 & 0x0000E001C0038007);
    let x4 = ((x3 & 0x0100800100020004) << 4)  | (x3 & 0x00C06180C3018603);
             ((x4 & 0x0080410082010402) << 2)  | (x4 & 0x1048209041208241)
}


pub fn unspread3_21(x0: u64) -> u32 {
    // 0b0001001001001001001001001001001001001001001001001001001001001001
    // 0b0001000011001000011000011001000011000011001000011000011001000011
    // 0b0000000111000000111000011000000111000011000000111000011000000111
    // 0b0000000000000111111000000000011111000000000011111000000000011111
    // 0b0000000000000000000000011111111111000000000000000000001111111111
    // 0b0000000000000000000000000000000000000000000111111111111111111111
    let x1 = ((x0 & 0x0201040208041008) >> 2)  | (x0 & 0x1048209041208241);
    let x2 = ((x1 & 0x1008001000200040) >> 4)  | (x1 & 0x00C06180C3018603);
    let x3 = ((x2 & 0x01C0018003000600) >> 6)  | (x2 & 0x0000E001C0038007);
    let x4 = ((x3 & 0x0007E000000F8000) >> 10) | (x3 & 0x00000007C000001F);
             (((x4 & 0x000001FFC0000000) >> 20) | (x4 & 0x00000000000003FF)) as u32
}


// Spread each bit of input out to every third bit of the output. This spreads the 5
// least-significant bits of the input into a 15-bit result.
pub fn spread3_5(x0: u16) -> u16 {
    // 0b0000000000011111
    let x1 = ((x0 & 0x0018) << 6) | (x0 & 0x0007);
    // 0b0000011000000111
    let x2 = ((x1 & 0x0004) << 4) | (x1 & 0x8603);
    // 0b0000011001000011
             ((x2 & 0x0402) << 2) | (x2 & 0x8241)
    // 0b0001001001001001
}

// A wide version of SpreadBits3_15(), operating on 4 input values at once.
// Intended for byte-packed coordinates.
pub fn spread3_5x4(x0: u64) -> u64 {
    // 0b0000000000011111 0000000000011111 0000000000011111 0000000000011111
    let x1 = ((x0 & 0x0018001800180018) << 6) | (x0 & 0x0007000700070007);
    // 0b0000011000000111 0000011000000111 0000011000000111 0000011000000111
    let x2 = ((x1 & 0x0004000400040004) << 4) | (x1 & 0x8603860386038603);
    // 0b0000011001000011 0000011001000011 0000011001000011 0000011001000011
             ((x2 & 0x0402040204020402) << 2) | (x2 & 0x8241824182418241)
    // 0b0001001001001001 0001001001001001 0001001001001001 0001001001001001
}


// Convert to and from Morton numbers.
// Naming convention is (morton|unmorton)(DIM)d(SIZE), where DIM is the number of dimensions and
// SIZE is the total number of bits for all components.

// Generate 16.16-bit Morton code
pub fn morton2d32(x: u32, y: u32) -> u32 {
    spread2_16(x) | (spread2_16(y) << 1)
}

pub fn unmorton2d32(x: u32) -> (u32, u32) {
    (unspread2_16(x & 0b01010101_01010101_01010101_01010101),
     unspread2_16((x & 0b10101010_10101010_10101010_10101010) >> 1))
}


// Generate 21.21.21-bit Morton code
pub fn morton3d63(x: u32, y: u32, z: u32) -> u64 {
    spread3_21(x) | (spread3_21(y) << 1) | (spread3_21(z) << 2)
}

pub fn unmorton3d63(m: u64) -> (u32, u32, u32) {
    (unspread3_21(m & MORTON3_21_X),
     unspread3_21((m & MORTON3_21_Y) >> 1),
     unspread3_21((m & MORTON3_21_Z) >> 2))
}


// Generate 7.7.7-bit Morton code
pub fn morton3d21(x: u32, y: u32, z: u32) -> u32 {
    let tmp = spread3_21((z << 14) | (y << 7) | x);
    (((tmp >> 42) | (tmp >> 21) | tmp) & 0x1FFFFF) as u32
}

pub fn unmorton3d21(m: u32) -> (u32, u32, u32) {
    let m64 = m as u64;
    let tmp = unspread3_21(((m64 & MORTON3_21_X) << 42) | ((m64 & MORTON3_21_Y) << 21) | (m64 & MORTON3_21_X));
    (tmp & 0x7F, (tmp >> 7) & 0x7F, (tmp >> 14) & 0x7F)
}


// Mathematical operations on Morton numbers.
fn morton_inc<T>(m: T, dim: T) -> T
    where T: Add<Output = T> + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T> +
        num::traits::Num + Copy
{
    (((m | !dim) + T::one() & dim) | (m & !dim))
}

fn morton_dec<T>(m: T, dim: T) -> T
    where T: Add<Output = T> + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T> +
        num::traits::Num + Copy
{
    (((m & dim) - T::one()) & dim) | (m & !dim)
}

fn morton_neq<T>(l: T, r: T, dim: T) -> bool
    where T: BitAnd<Output = T> + num::traits::Num + Copy + PartialOrd
{
    (l & dim) != (r & dim)
}

fn morton_eq<T> (l: T, r: T, dim: T) -> bool
    where T: BitAnd<Output = T> + num::traits::Num + Copy + PartialOrd
{
    (l & dim) == (r & dim)
}

fn morton_lt<T> (l: T, r: T, dim: T) -> bool
    where T: BitAnd<Output = T> + num::traits::Num + Copy + PartialOrd
{
    (l & dim) < (r & dim)
}

fn morton_gt<T> (l: T, r: T, dim: T) -> bool
    where T: BitAnd<Output = T> + num::traits::Num + Copy + PartialOrd
{
    (l & dim) > (r & dim)
}

