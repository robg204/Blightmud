use std::fmt;

pub struct ScrollRegion(pub u16, pub u16);
impl fmt::Display for ScrollRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{};{}r", self.0, self.1)
    }
}

pub struct ResetScrollRegion;
impl fmt::Display for ResetScrollRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[r")
    }
}

pub struct DisableOriginMode;
impl fmt::Display for DisableOriginMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[?6l")
    }
}
