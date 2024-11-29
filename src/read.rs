use std::io::{self, Read};

use crate::{endianness::Endianness, from_bytes::FromBytes};

pub trait Readable: Sized {
    fn read_from<R: Read>(read: R, endianness: Endianness) -> io::Result<Self>;
}
impl<F: FromBytes> Readable for F
where
    [u8; F::SIZE]:,
{
    fn read_from<R: Read>(mut read: R, endianness: Endianness) -> io::Result<Self> {
        let mut buf = [0; F::SIZE];
        read.read(&mut buf)?;
        Ok(F::from_bytes(buf, endianness))
    }
}
pub trait ReadExt {
    fn read_object<R: Readable>(&mut self, endianness: Endianness) -> io::Result<R>;
}
impl<R: Read> ReadExt for R {
    fn read_object<O: Readable>(&mut self, endianness: Endianness) -> io::Result<O> {
        O::read_from(self, endianness)
    }
}
