use std::ops::{BitAnd, BitOr, BitXor, Not, BitAndAssign, BitOrAssign, BitXorAssign};

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
    fn eq(&self, rhs: &Line) -> bool {
        match (self, rhs) {
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
    fn bitand(self, rhs: Line) -> Line {
        match (self, rhs) {
            (Line::High, Line::High) => Line::High,
            _ => Line::Low
        }
    }
}

impl BitOr for Line {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Line) -> Line {
        match (self, rhs) {
            (Line::Disconnected, Line::Disconnected) | (Line::Disconnected, Line::Low) |
            (Line::Low, Line::Disconnected) | (Line::Low, Line::Low) => Line::Low,
            _ => Line::High
        }
    }
}

impl BitXor for Line {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Line) -> Line {
        match (self, rhs) {
            (Line::High, Line::Low) | (Line::High, Line::Disconnected) |
            (Line::Low, Line::High) | (Line::Disconnected, Line::High) => Line::High,
            _ => Line::Low
        }
    }
}

macro_rules! forward_line_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<$t> for &'a $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self, rhs: $t) -> <$t as $imp>::Output {
                $imp::$method(*self, rhs)
            }
        }

        impl<'a> $imp<&'a $t> for $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self, rhs: &'a $t) -> <$t as $imp>::Output {
                $imp::$method(self, *rhs)
            }
        }

        impl<'a, 'b> $imp<&'b $t> for &'a $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self, rhs: &'b $t) -> <$t as $imp>::Output {
                $imp::$method(*self, *rhs)
            }
        }
    }
}

forward_line_impl!{impl BitAnd, bitand for Line}
forward_line_impl!{impl BitOr, bitor for Line}
forward_line_impl!{impl BitXor, bitxor for Line}

impl BitAndAssign for Line {
    fn bitand_assign(&mut self, rhs: Line) {
        match (*self, rhs) {
            (Line::High, Line::High) => *self = Line::High,
            _ => *self = Line::Low
        }
    }
}

impl BitOrAssign for Line {
    fn bitor_assign(&mut self, rhs: Line) {
        match (*self, rhs) {
            (Line::Low, Line::Low) | (Line::Disconnected, Line::Disconnected) => *self = Line::Low,
            _ => *self = Line::High
        }
    }
}

impl BitXorAssign for Line {
    fn bitxor_assign(&mut self, rhs: Line) {
        match (*self, rhs) {
            (Line::High, Line::Low) | (Line::High, Line::Disconnected) |
            (Line::Low, Line::High) | (Line::Disconnected, Line::High) => *self = Line::High,
            _ => *self = Line::Low
        }
    }
}

macro_rules! forward_assign_line_impl {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp<&'a $t> for $t {
            fn $method(&mut self, rhs: &'a $t) {
                $imp::$method(self, *rhs);
            }
        }
    }
}

forward_assign_line_impl!{impl BitAndAssign, bitand_assign for Line}
forward_assign_line_impl!{impl BitOrAssign, bitor_assign for Line}
forward_assign_line_impl!{impl BitXorAssign, bitxor_assign for Line}

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