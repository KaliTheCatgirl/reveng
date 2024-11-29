use crate::{endianness::Endianness, from_bytes::FromBytes};

pub mod strings {
    use std::{
        io::{self, Read},
        marker::PhantomData,
    };

    use crate::{endianness::Endianness, from_bytes::FromBytes, read::Readable};

    /// A null-terminated string.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct NullString(pub String);
    impl Readable for NullString {
        fn read_from<R: Read>(mut read: R, _: Endianness) -> io::Result<Self> {
            let mut buf = [0];
            let mut string = String::new();
            while read.read(&mut buf)? != 0 && buf[0] != 0 {
                string.push(buf[0] as char);
            }
            Ok(Self(string))
        }
    }

    /// A null-terminated wide string.
    #[derive(Debug)]
    #[allow(unused)]
    pub struct WideNullString(pub String);
    impl Readable for WideNullString {
        fn read_from<R: Read>(mut read: R, endianness: Endianness) -> io::Result<Self> {
            let mut buf = [0; 2];
            let mut string = String::new();
            while read.read(&mut buf)? >= 2 && buf != [0, 0] {
                string.push(
                    char::from_u32(u16::from_bytes(buf, endianness) as u32).unwrap_or('\u{FFFD}'),
                );
            }
            Ok(Self(string))
        }
    }

    /// A fixed-size string.
    #[derive(Debug)]
    #[allow(unused)]
    pub struct FixedString<const N: usize>(pub String);
    impl<const N: usize> FromBytes for FixedString<N> {
        const SIZE: usize = N;
        fn from_bytes(bytes: [u8; Self::SIZE], _: Endianness) -> Self {
            Self(String::from_utf8_lossy(&bytes).to_string())
        }
    }

    /// A fixed-size wide string.
    #[derive(Debug)]
    #[allow(unused)]
    pub struct WideFixedString<const N: usize>(pub String);
    impl<const N: usize> FromBytes for WideFixedString<N> {
        const SIZE: usize = N * 2;
        fn from_bytes(bytes: [u8; Self::SIZE], endianness: Endianness) -> Self {
            let mut characters = [0; N];
            for (index, character) in characters.iter_mut().enumerate() {
                *character = u16::from_bytes([bytes[index * 2], bytes[index * 2 + 1]], endianness);
            }
            Self(String::from_utf16_lossy(&characters).to_string())
        }
    }

    /// A string with its length specified before the data in a given format.
    #[derive(Debug)]
    #[allow(unused)]
    pub struct LengthedString<T: num::ToPrimitive + Readable, const MAX_LENGTH: usize = { usize::MAX }>(
        pub String,
        PhantomData<T>,
    );
    impl<T: num::ToPrimitive + Readable, const MAX_LENGTH: usize> Readable
        for LengthedString<T, MAX_LENGTH>
    {
        fn read_from<R: Read>(mut read: R, endianness: Endianness) -> io::Result<Self> {
            let length = T::read_from(&mut read, endianness)?.to_usize().unwrap_or(0);
            if length > MAX_LENGTH {
                return Err(io::Error::other("Exceeded maximum lengthed string size!"));
            }
            let mut buf = vec![0; length];
            read.read(&mut buf)?;
            Ok(Self(String::from_utf8_lossy_owned(buf), PhantomData))
        }
    }

    /// A wide string with its length specified before the data in a given format.
    ///
    /// The length in question specifies the number of characters to appear in the string, and not bytes.
    #[derive(Debug)]
    #[allow(unused)]
    pub struct WideLengthedString<
        T: num::ToPrimitive + Readable,
        const MAX_LENGTH: usize = { usize::MAX },
    >(pub String, PhantomData<T>);
    impl<T: num::ToPrimitive + Readable, const MAX_LENGTH: usize> Readable
        for WideLengthedString<T, MAX_LENGTH>
    {
        fn read_from<R: Read>(mut read: R, endianness: Endianness) -> io::Result<Self> {
            let length = T::read_from(&mut read, endianness)?.to_usize().unwrap_or(0);
            if length > MAX_LENGTH {
                return Err(io::Error::other(
                    "Exceeded maximum wide lengthed string size!",
                ));
            }
            let mut buf = vec![0; length * 2];
            read.read(&mut buf)?;
            let mut characters = vec![0; length];
            for (index, character) in characters.iter_mut().enumerate() {
                *character = u16::from_bytes([buf[index * 2], buf[index * 2 + 1]], endianness);
            }
            Ok(Self(String::from_utf8_lossy_owned(buf), PhantomData))
        }
    }
}

/// Holds nothing, used for discarding unneeded data.
#[derive(Debug)]
pub struct Padding<const N: usize>;
impl<const N: usize> FromBytes for Padding<N> {
    const SIZE: usize = N;
    fn from_bytes(_: [u8; Self::SIZE], _: Endianness) -> Self {
        Self
    }
}
