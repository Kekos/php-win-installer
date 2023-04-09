use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Arch {
    X86,
    X64,
    Unsupported,
}

impl Arch {
    pub fn get() -> Arch {
        #[cfg(target_arch = "x86")]
        {
            Arch::X86
        }

        #[cfg(target_arch = "x86_64")]
        {
            Arch::X64
        }

        Arch::Unsupported
    }
}

impl ToString for Arch {
    fn to_string(&self) -> String {
        String::from(match self {
            Arch::X86 => "x86",
            Arch::X64 => "x64",
            Arch::Unsupported => "unsupported",
        })
    }
}
