pub use ascii::{AsciiChar as achar, AsciiStr as astr, AsciiString as AString};

#[macro_export]
macro_rules! astr {
    ($str:literal) => {
        $crate::ascii::init_astr($str)
    };
}

pub const fn init_astr(bytes: &'static [u8]) -> &'static astr {
    validate_slice(bytes.as_ptr(), bytes.len());
    unsafe { std::mem::transmute(bytes) }
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
    fn split_str<'src, 'pat>(&'src self, separator: &'pat astr) -> Split<'src, 'pat>;
}

impl AsciiStrExt for astr {
    fn get(&self, index: usize) -> Option<achar> {
        if index >= self.len() {
            None
        } else {
            Some(self[index])
        }
    }

    fn split_str<'src, 'pat>(&'src self, separator: &'pat astr) -> Split<'src, 'pat> {
        assert!(!separator.is_empty());
        Split {
            remainder: Some(self),
            pattern: separator,
        }
    }
}

#[derive(Clone)]
pub struct Split<'src, 'pat> {
    remainder: Option<&'src astr>,
    pattern: &'pat astr,
}

impl<'src, 'pat> Iterator for Split<'src, 'pat> {
    type Item = &'src astr;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;
        let pattern_len = self.pattern.len();

        let pattern_at_index = 'index: loop {
            for i in 0..remainder.len() - pattern_len.min(remainder.len()) {
                let slice = &remainder[i..i + pattern_len];
                if slice == self.pattern {
                    break 'index i;
                }
            }
            self.remainder = None;
            return Some(remainder);
        };
        let result = &remainder[0..pattern_at_index];
        self.remainder = Some(&remainder[pattern_at_index + pattern_len..]);
        Some(result)
    }
}
