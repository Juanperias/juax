use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegError {
    #[error("Register {0} not exist")]
    RegNotExist(u32)
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq)]
pub enum Reg {
    A0,
    A1
}

macro_rules! from_hex {
    ($($number:expr, $variant:ident),*) => {
            impl TryFrom<u32> for Reg {
                type Error = RegError;

                fn try_from(value: u32) -> Result<Self, Self::Error> {
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
