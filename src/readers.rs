use std::{borrow::Cow, io::{self, Read}};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct MemRead<'data> {
    data: Cow<'data, [u8]>,
    offset: usize,
}
#[allow(dead_code)]
impl<'data> MemRead<'data> {
    pub fn new(data: impl Into<Cow<'data, [u8]>>) -> Self {
        Self::with_offset(data, 0)
    }
    pub fn with_offset(data: impl Into<Cow<'data, [u8]>>, offset: usize) -> Self {
        Self {
            data: data.into(),
            offset,
        }
    }
    pub fn set_offset(&mut self, new_offset: usize) {
        self.offset = new_offset;
    }
}
impl<'data> Read for MemRead<'data> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = buf.len().min(self.data.len().saturating_sub(self.offset));
        buf[..size].copy_from_slice(&self.data[self.offset..self.offset + size]);
        self.offset += size;
        Ok(size)
    }
}

pub trait ToMemReader<'data> {
    fn to_mem_reader(self) -> MemRead<'data>;
}
impl<'data, B: Into<Cow<'data, [u8]>>> ToMemReader<'data> for B {
    fn to_mem_reader(self) -> MemRead<'data> {
        MemRead::new(self.into())
    }
}
