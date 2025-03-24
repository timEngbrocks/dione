pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;

pub trait Width {
    fn width(&self) -> usize;
}

pub trait MAX {
    fn max_value() -> Self;
}

impl MAX for U1 {
    fn max_value() -> Self {
        U1::MAX
    }
}

impl MAX for U2 {
    fn max_value() -> Self {
        U2::MAX
    }
}

impl MAX for U4 {
    fn max_value() -> Self {
        U4::MAX
    }
}

pub trait ZERO {
    fn zero_value() -> Self;
}

impl ZERO for U1 {
    fn zero_value() -> Self {
        0
    }
}

impl ZERO for U2 {
    fn zero_value() -> Self {
        0
    }
}

impl ZERO for U4 {
    fn zero_value() -> Self {
        0
    }
}
