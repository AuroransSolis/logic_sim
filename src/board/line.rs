use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Copy, Clone, Eq, Debug)]
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

impl Into<bool> for Line {
    fn into(self) -> bool {
        match self {
            Line::High => true,
            _ => false
        }
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

impl PartialEq for Line {
    fn eq(self, other: Line) -> bool {
        match (self, other) {
            (Line::High, Line::High) => true,
            (Line::Low, Line::Low) |
            (Line::Low, Line::Disconnected) |
            (Line::Disconnected, Line::Low) |
            (Line::Disconnected, Line::Disconnected) => true,
            _ => false
        }
    }
}

impl BitAnd for Line {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Line) -> Line {
        match (self, other) {
            (Line::High, Line::High) => Line::High,
            _ => Line::Low
        }
    }
}

impl BitOr for Line {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Line) -> Line {
        match (self, other) {
            (Line::Disconnected, Line::Disconnected) | (Line::Disconnected, Line::Low) |
            (Line::Low, Line::Disconnected) | (Line::Low, Line::Low) => Line::Low,
            _ => Line::High
        }
    }
}

impl BitXor for Line {
    type Output = Self;

    #[inline]
    fn bitxor(self, other: Line) -> Line {
        match (self, other) {
            (Line::High, Line::High) | (Line::Low, Line::Low) |
            (Line::Disconnected, Line::Disconnected) | (Line::Low, Line::Disconnected) |
            (Line::Disconnected, Line::Low) => Line::Low,
            (Line::High, Line::Low) | (Line::Low, Line::High) |
            (Line::High, Line::Disconnected) | (Line::Disconnected, Line::High) => Line::High
        }
    }
}

macro_rules! forward_line_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<$t> for &'a $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self, other: $t) -> <$t as $imp>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $t> for $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self, other: &'a $t) -> <$t as $imp>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'b $t> for &'a $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self, other: &'b $t) -> <$t as $imp>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

forward_line_impl!{impl BitAnd, bitand for Line}
forward_line_impl!{impl BitOr, bitor for Line}
forward_line_impl!{impl BitXor, bitxor for Line}

impl Not for Line {
    type Output = Self;

    #[inline]
    fn not(self) -> Self {
        match self {
            Line::Disconnected | Line::Low => Line::High,
            Line::High => Line::Low
        }
    }
}

impl<'a> Not for &'a Line {
    type Output = Line;

    #[inline]
    fn not(self) -> Line {
        Not::not(*self)
    }
}