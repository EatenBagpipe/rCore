use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Kill {
    pub pid: i32,
    pub uid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Fields {
    // make initialization pass compilation
    _init: [u8; 0],
    kill: Kill,
    // TODO: fill follow the ref: https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/include/uapi/asm-generic/siginfo.h?h=v5.4.46#n32
}

impl Fields {
    pub fn kill_mut(&mut self) -> &mut Kill {
        unsafe { &mut self.kill }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InfoInner {
    pub signo: i32,
    pub errno: i32,
    pub code: i32,
    fields: Fields,
}

impl Deref for InfoInner {
    type Target = Fields;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl DerefMut for InfoInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fields
    }
}

const SI_MAX_SIZE: usize = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Siginfo {
    inner: ManuallyDrop<InfoInner>,
    pad: [u8; SI_MAX_SIZE],
}

impl Siginfo {
    pub fn new(signo: i32, errno: i32, code: i32) -> Self {
        Siginfo {
            inner: ManuallyDrop::new(InfoInner {
                signo,
                errno,
                code,
                fields: Fields {
                    _init: [0; 0],
                }
            })
        }
    }
}

impl Deref for Siginfo {
    type Target = InfoInner;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.inner }
    }
}

impl DerefMut for Siginfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.inner }
    }
}