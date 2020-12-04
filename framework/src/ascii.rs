pub use ascii::{AsciiChar as achar, AsciiStr as astr, AsciiString as AString};

#[macro_export]
macro_rules! astr {
    ($str:literal) => {
        $crate::ascii::init_astr($str)
    };
}

pub const fn init_astr(bytes: &'static [u8]) -> &'static astr {
    validate_slice(bytes.as_ptr(), bytes.len());
    unsafe {
        std::mem::transmute(bytes)
    }
}

const fn validate_slice(bytes: *const u8, size: usize) {
    let mut i = 0;
    loop {
        if i >= size {
            break;
        }
        unsafe {
            let byte = *bytes.add(i);
            if byte >= 128 {
                panic!("invalid ASCII character");
            }
        }
        i += 1;
    }
}

pub trait AsciiStrExt {
    fn get(&self, index: usize) -> Option<achar>;
}

impl AsciiStrExt for astr {
    fn get(&self, index: usize) -> Option<achar> {
        if index >= self.len() {
            None
        } else {
            Some(self[index])
        }
    }
}
