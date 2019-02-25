use std::ops::{Add, Mul, Not, BitXor};

#[derive(Copy, Clone, Eq, Debug)]
pub enum Line {
    High,
    Low,
    Disconnected
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

pub fn not(l0: Line) -> Line {
    match l0 {
        Line::High => Line::Low,
        _ => Line::High
    }
}

#[inline]
pub fn and(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (Line::High, Line::High) => Line::High,
        _ => Line::Low
    }
}

#[inline]
pub fn or(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (Line::High, _) | (_, Line::High) => Line::High,
        _ => Line::Low
    }
}

#[inline]
pub fn xor(l0: Line, l1: Line) -> Line {
    match l0 {
        Line::High => not(l1),
        Line::Low | Line::Disconnected => l1
    }
}

#[inline]
pub fn nand(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (Line::High, Line::High) => Line::Low,
        _ => Line::High
    }
}

#[inline]
pub fn nor(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (Line::High, _) | (_, Line::High) => Line::Low,
        _ => Line::High
    }
}

#[inline]
pub fn xnor(l0: Line, l1: Line) -> Line {
    match l0 {
        Line::High => l1,
        Line::Low | Line::Disconnected => not(l1)
    }
}

impl Line {
    pub fn is_high(&self) -> bool {
        match self {
            &Line::High => true,
            _ => false
        }
    }

    pub fn is_low(&self) -> bool {
        match self {
            &Line::Low => true,
            _ => false
        }
    }

    pub fn is_disconnected(&self) -> bool {
        match self {
            &Line::Disconnected => true,
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

impl Into<bool> for Line {
    fn into(self) -> bool {
        match self {
            Line::High => true,
            _ => false
        }
    }
}

impl Not for Line {
    type Output = Line;

    fn not(self) -> Self {
        not(self)
    }
}

impl<'a> Not for &'a Line {
    type Output = Line;

    fn not(self) -> Line {
        not(*self)
    }
}

macro_rules! lmao_im_lazy {
    ($($impl_trait:ident, $method:ident, $function:ident)|*) => {$(
        impl<'a> $impl_trait<&'a Line> for Line {
            type Output = Line;

            fn $method(self, rhs: &'a Line) -> Line {
                $function(self, *rhs)
            }
        }

        impl<'a> $impl_trait<Line> for &'a Line {
            type Output = Line;

            fn $method(self, rhs: Line) -> Line {
                $function(*self, rhs)
            }
        }

        impl<'a, 'b> $impl_trait<&'b Line> for &'a Line {
            type Output = Line;

            fn $method(self, rhs: &'b Line) -> Line {
                $function(*self, *rhs)
            }
        }
    )*}
}

impl Mul for Line {
    type Output = Line;

    fn mul(self, rhs: Line) -> Self {
        and(self, rhs)
    }
}

impl Add for Line {
    type Output = Line;

    fn add(self, rhs: Line) -> Self {
        or(self, rhs)
    }
}

impl BitXor for Line {
    type Output = Line;

    fn bitxor(self, rhs: Line) -> Self {
        xor(self, rhs)
    }
}

lmao_im_lazy!{Mul, mul, and | Add, add, or | BitXor, bitxor, xor}