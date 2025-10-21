use std::io;
use std::io::Write;

pub trait Ansi {
    fn as_str(&self) -> &str;
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
