use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegError {
    #[error("Register {0} not exist")]
    RegNotExist(u32),

    #[error("Register {0} not exist")]
    RegNotExistStr(String),
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq)]
pub enum Reg {
    A0,
    A1,
}

impl TryFrom<String> for Reg {
    type Error = RegError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let reg = match value.as_str() {
            "a0" => Reg::A0,
            "a1" => Reg::A1,
            _ => return Err(RegError::RegNotExistStr(value)),
        };

        Ok(reg)
    }
}

macro_rules! from_to_hex {
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

            impl Into<u32> for Reg {
                fn into(self) -> u32 {
                    match self {
                        $(
                            Self::$variant => $number,
                        )*
                    }
                }
            }
    };
}

from_to_hex!(0x01, A0, 0x02, A1);
