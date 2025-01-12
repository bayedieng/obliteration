use std::fmt::{Display, Formatter};

/// Privilege identifier.
///
/// See https://github.com/freebsd/freebsd-src/blob/release/9.1.0/sys/sys/priv.h for standard
/// FreeBSD privileges.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Privilege(i32);

impl Privilege {
    pub const MAXFILES: Self = Self(3);
    pub const PROC_SETLOGIN: Self = Self(161);
    pub const VFS_ADMIN: Self = Self(312);
    pub const SCE680: Self = Self(680);
    pub const SCE683: Self = Self(683);
    pub const SCE686: Self = Self(686);
}

impl Display for Privilege {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MAXFILES => f.write_str("PRIV_MAXFILES"),
            Self::PROC_SETLOGIN => f.write_str("PRIV_PROC_SETLOGIN"),
            Self::VFS_ADMIN => f.write_str("PRIV_VFS_ADMIN"),
            Self::SCE680 => f.write_str("SCE680"),
            Self::SCE683 => f.write_str("SCE683"),
            Self::SCE686 => f.write_str("SCE686"),
            v => v.0.fmt(f),
        }
    }
}
