use super::error::Error;
use std::mem;

/// A struct representing a slice from which values implementing `Decode` can be read.
pub struct Reader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    /// Constructs a new `Reader` with the provided data slice.
    pub fn new(data: &'a [u8]) -> Reader<'a> {
        Reader { data, offset: 0 }
    }

    /// Reads bytes to a buffer.
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        if self.data.len() - self.offset < buffer.len() {
            return Err(Error::Overflow);
        }

        buffer.copy_from_slice(&self.data[self.offset..self.offset + buffer.len()]);
        self.offset += buffer.len();

        Ok(())
    }

    /// Reads a value implementing `Decode` from the data slice and advances its offset.
    pub fn read<T>(&mut self) -> Result<T, Error>
    where
        T: Decode,
    {
        T::decode(self)
    }

    /// Reads a value implementing `Decode` from the data slice at a specified offset.
    pub fn read_at<T>(&self, offset: usize) -> Result<T, Error>
    where
        T: Decode,
    {
        if offset >= self.data.len() {
            return Err(Error::Overflow);
        }

        Reader {
            data: self.data,
            offset,
        }
        .read()
    }
}

/// A trait representing values that can be read from a `Reader`.
pub trait Decode: Sized {
    fn decode(reader: &mut Reader) -> Result<Self, Error>;

    /// Creates a temporary `Reader` and reads itself.
    ///
    /// Convenience method.
    fn decode_in_place(data: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(data);
        reader.read()
    }
}

macro_rules! impl_decode_number {
    ($($ty:ty)*) => {
        $(impl Decode for $ty {
            fn decode(reader: &mut Reader) -> Result<$ty, Error> {
                let mut bytes = [0u8; mem::size_of::<$ty>()];
                reader.read_bytes(&mut bytes)?;

                Ok(<$ty>::from_le_bytes(bytes))
            }
        })*
    };
}

impl_decode_number!(i8 u8 i16 u16 i32 u32 i64 u64 f32 f64);

macro_rules! impl_decode_array {
    ($($length:expr)*) => {
        $(impl<T> Decode for [T; $length]
        where
            T: Default + Clone + Copy + Decode
        {
            fn decode(reader: &mut Reader) -> Result<[T; $length], Error> {
                let mut data = [Default::default(); $length];
                for elem in &mut data {
                    *elem = reader.read()?;
                }

                Ok(data)
            }
        })*
    };
}

impl_decode_array!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
