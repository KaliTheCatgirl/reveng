use crate::endianness::Endianness;

pub trait FromBytes {
    const SIZE: usize;
    fn from_bytes(bytes: [u8; Self::SIZE], endianness: Endianness) -> Self;
}
impl FromBytes for u8 {
    const SIZE: usize = 1;
    fn from_bytes(bytes: [u8; 1], _: Endianness) -> Self {
        bytes[0]
    }
}
impl FromBytes for i8 {
    const SIZE: usize = 1;
    fn from_bytes(bytes: [u8; 1], _: Endianness) -> Self {
        bytes[0] as i8
    }
}
macro_rules! impl_from_bytes_integer {
    ($type:tt, $size:expr) => {
        impl FromBytes for $type {
            const SIZE: usize = $size;
            fn from_bytes(bytes: [u8; $size], endianness: Endianness) -> Self {
                match endianness {
                    Endianness::Little => $type::from_le_bytes(bytes),
                    Endianness::Native => $type::from_ne_bytes(bytes),
                    Endianness::Big => $type::from_be_bytes(bytes),
                }
            }
        }
    };
}
impl_from_bytes_integer!(u16, 2);
impl_from_bytes_integer!(u32, 4);
impl_from_bytes_integer!(u64, 8);
impl_from_bytes_integer!(u128, 16);

impl_from_bytes_integer!(i16, 2);
impl_from_bytes_integer!(i32, 4);
impl_from_bytes_integer!(i64, 8);
impl_from_bytes_integer!(i128, 16);

impl_from_bytes_integer!(f32, 4);
impl_from_bytes_integer!(f64, 8);
