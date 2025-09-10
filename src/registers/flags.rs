#[derive(Debug, Clone)]
pub enum ConditionalFlag {
    Postitive = 1 << 0, /* P */
    Zero = 1 << 1,      /* Z */
    Negative = 1 << 2,  /* N */
}

pub trait ConditionalFlagtrait {
    fn from_value(reg: u16) -> Self;
}

impl ConditionalFlagtrait for ConditionalFlag {
    fn from_value(value: u16) -> Self {
        if value == 0 {
            Self::Zero
        } else if (value >> 15) != 0 {
            Self::Negative
        } else {
            Self::Postitive
        }
    }
}
