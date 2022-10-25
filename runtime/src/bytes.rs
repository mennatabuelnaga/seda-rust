use std::ops::Deref;

use error_stack::IntoReport;

use super::Result;

#[derive(Clone)]
pub struct Bytes(Vec<u8>);

/// We implement Deref over the Bytes type allowing us to avoid a clone for
/// `FromBytes`.
impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

pub trait ToBytes {
    fn to_bytes(self) -> Bytes;
}

pub trait FromBytes
where
    Self: Sized,
{
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}

impl ToBytes for String {
    fn to_bytes(self) -> Bytes {
        Bytes(self.as_bytes().to_vec())
    }
}

impl FromBytes for String {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(std::str::from_utf8(bytes).into_report()?.into())
    }
}

macro_rules! bytes_impls_le_bytes {
    ($type_:ty, $num_bytes:expr) => {
        impl ToBytes for $type_ {
            fn to_bytes(self) -> Bytes {
                Bytes(self.to_le_bytes().to_vec())
            }
        }

        impl FromBytes for $type_ {
            fn from_bytes(bytes: &[u8]) -> Result<Self> {
                let bytes: [u8; $num_bytes] = bytes.try_into().into_report()?;
                Ok(<$type_>::from_le_bytes(bytes))
            }
        }
    };
}

bytes_impls_le_bytes!(u8, 1);
bytes_impls_le_bytes!(u32, 4);
bytes_impls_le_bytes!(u64, 8);
bytes_impls_le_bytes!(u128, 16);
bytes_impls_le_bytes!(i8, 1);
bytes_impls_le_bytes!(i32, 4);
bytes_impls_le_bytes!(i64, 8);
bytes_impls_le_bytes!(i128, 16);
bytes_impls_le_bytes!(f32, 4);
bytes_impls_le_bytes!(f64, 8);
