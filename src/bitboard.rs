use square::*;
use std::ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Mul, Not};
use std::fmt;
use rank::Rank;
use file::File;

/// A good old-fashioned bitboard
/// You *do* have access to the actual value, but you are probably better off
/// using the implemented operators to work with this object.
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct BitBoard(pub u64);

/// An empty bitboard
pub const EMPTY: BitBoard = BitBoard(0);

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

impl Mul for BitBoard {
    type Output = BitBoard;

    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        for x in 0..64 {
            if self.0 & (1u64 << x) == (1u64 << x) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
            if x % 8 == 7 {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}

impl BitBoard {
    /// Construct a new bitboard from a u64
    pub fn new(b: u64) -> BitBoard {
        BitBoard(b)
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    pub fn set(rank: Rank, file: File) -> BitBoard {
        BitBoard::from_square(Square::make_square(rank, file))
    }

    /// Construct a new `BitBoard` with a particular `Square` set
    pub fn from_square(sq: Square) -> BitBoard {
        BitBoard(1u64 << sq.to_int())
    }

    /// Convert an `Option<Square>` to an `Option<BitBoard>`
    pub fn from_maybe_square(sq: Option<Square>) -> Option<BitBoard> {
        sq.map(|s| BitBoard::from_square(s))
    }

    /// Convert a `BitBoard` to a `Square`.  This grabs the least-significant `Square`
    pub fn to_square(&self) -> Square {
        unsafe {
            Square::new(self.0.trailing_zeros() as u8)
        }
    }

    /// Count the number of `Squares` set in this `BitBoard`
    pub fn popcnt(&self) -> u32 {
        self.0.count_ones()
    }

    /// Reverse this `BitBoard`.  Look at it from the opponents perspective.
    pub fn reverse_colors(&self) -> BitBoard {
        BitBoard(self.0.swap_bytes())
    }

    /// Convert this `BitBoard` to a `usize` (for table lookups)
    pub fn to_size(&self, rightshift: u8) -> usize {
        (self.0 >> rightshift) as usize
    }
}

/// For the `BitBoard`, iterate over every `Square` set.
impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
        if self.0 == 0 {
            None
        } else {
            let result = self.to_square();
            *self ^= BitBoard::from_square(result);
            Some(result)
        }
    }
}

