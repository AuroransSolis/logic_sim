use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Line {
    High,
    Low,
    Disconnected
}

macro_rules! impl_is_variant {
    ($enm:ty {
        $($variant:pat => $method_name:ident),*
    }) => {
        impl $enm {
            $(
            pub fn $method_name(&self) -> bool {
                match self {
                    $variant => true,
                    _ => false
                }
            }
            )*
        }
    }
}

impl_is_variant!{Line {
    Line::High => is_high,
    Line::Low => is_low,
    Line::Disconnected => is_disconnected
}}

impl Line {
    pub fn try_into_bool(self) -> Option<bool> {
        if self.is_high() {
            Some(true)
        } else if self.is_low() {
            Some(false)
        } else {
            None
        }
    }

    pub fn into_bool(self) -> bool {
        debug_assert!(!self.is_disconnected());
        if self.is_high() {
            true
        } else {
            false
        }
    }

    pub fn high() -> Self {
        Line::High
    }

    pub fn low() -> Self {
        Line::Low
    }

    pub fn disconnected() -> Self {
        Line::Disconnected
    }
}

impl From<bool> for Line {
    fn from(other: bool) -> Self {
        if other {
            Line::High
        } else {
            Line::Low
        }
    }
}

impl Into<bool> for Line {
    fn into(self) -> bool {
        match self {
            Line::High => true,
            Line::Low => false,
            Line::Disconnected => unimplemented!()
        }
    }
}

#[inline]
impl BitAnd for Line {
    fn bitand(self, other: Line) -> Line {
        (bool::from(self) && bool::from(other)).into()
    }
}

#[inline]
impl BitOr for Line {
    fn bitor(self, other: Line) -> Line {
        (bool::from(self) || bool::from(other)).into()
    }
}

#[inline]
impl BitXor for Line {
    fn bitxor(self, other: Line) -> Line {
        (self != other).into()
    }
}

macro_rules! forward_line_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        #[inline]
        impl<'a> $imp<$t> for &'a $t {
            type Output = <$t as $imp>::Output;

            fn $method(self, other: $t) -> <$t as $imp>::Output {
                $imp::$method(*self, other)
            }
        }

        #[inline]
        impl<'a> $imp<&'a $t> for $t {
            type Output = <$t as $imp>::Output;

            fn $method(self, other &'a $t) -> <$t as $imp>::Output {
                $imp::$method(self, *other)
            }
        }

        #[inline]
        impl<'a, 'b> $imp<&'b $t> for &'a $t {
            type output = <$t as $imp>::Output;

            fn $method(self, other: &'b $t) -> <$t as $imp>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

forward_line_impl!{impl BitAnd, bitand for Line}
forward_line_impl!{impl BitOr, bitor for Line}
forward_line_impl!{impl Bitxor, bitxor for Line}

#[inline]
impl Not for Line {
    type Output = Self;

    fn not(self) -> Self {
        (!bool::from(self)).into()
    }
}

#[inline]
impl<'a> Not for &'a Line {
    type Output = Line;

    fn not(self) -> Line {
        (!bool::from(*self)).into()
    }
}