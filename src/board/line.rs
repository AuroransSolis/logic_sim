use std::ops::{BitAnd, BitOr, BitXor, Not, BitAndAssign, BitOrAssign, BitXorAssign};

#[derive(Copy, Clone, Eq, Debug)]
pub enum Line {
    High,
    Low,
    Disconnected
}

pub(crate) const HIGH: Line = Line::High;
pub(crate) const LOW: Line = Line::Low;
pub(crate) const DISCONNECTED: Line = Line::Disconnected;

pub(crate) fn not(l0: Line) -> Line {
    match l0 {
        HIGH => LOW,
        _ => HIGH
    }
}

#[inline]
pub(crate) fn and(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (HIGH, HIGH) => HIGH,
        _ => LOW
    }
}

#[inline]
pub(crate) fn or(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (HIGH, _) | (_, HIGH) => HIGH,
        _ => LOW
    }
}

#[inline]
pub(crate) fn xor(l0: Line, l1: Line) -> Line {
    match l0 {
        HIGH => !l1,
        LOW | DISCONNECTED => l1
    }
}

#[inline]
pub(crate) fn nand(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (HIGH, HIGH) => LOW,
        _ => HIGH
    }
}

#[inline]
pub(crate) fn nor(l0: Line, l1: Line) -> Line {
    match (l0, l1) {
        (HIGH, _) | (_, HIGH) => LOW,
        _ => HIGH
    }
}

#[inline]
pub(crate) fn xnor(l0: Line, l1: Line) -> Line {
    match l0 {
        HIGH => l1,
        LOW | DISCONNECTED => !l1
    }
}