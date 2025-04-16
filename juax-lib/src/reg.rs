use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegError {
    #[error("Register {0} not exist")]
    RegNotExist(i32)
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq)]
pub enum Reg {
    A0,
    A1
}

macro_rules! from_hex {
    ($($number:expr, $variant:ident),*) => {
            impl TryFrom<i32> for Reg {
                type Error = RegError;

                fn try_from(value: i32) -> Result<Self, Self::Error> {
                    $(
                        if value == $number {
                            return Ok(Self::$variant);
                        }
                    )*   

                    Err(RegError::RegNotExist(value))
                }
            }
    };
}

from_hex!(
    0x01, A0,
    0x02, A1
);
