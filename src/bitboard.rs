use std::ops::{BitAnd, BitOr, BitXor, Neg, Shl, Shr};
use std::fmt::Display;

pub const A_FILE: Bitboard = Bitboard(0b1000000010000000100000001000000010000000100000001000000010000000);
pub const H_FILE: Bitboard = Bitboard(0b000100000001000000010000000100000001000000010000000100000001);
pub const SECOND_RANK: Bitboard = Bitboard(0b1111111100000000);
pub const FIRST_RANK: Bitboard = Bitboard(0b11111111);
pub const SEVENTH_RANK: Bitboard = Bitboard(0b11111111000000000000000000000000000000000000000000000000);
pub const EIGHT_RANK: Bitboard = Bitboard(0b1111111100000000000000000000000000000000000000000000000000000000);
pub struct Bitboard(u64);

 impl Bitboard{

    pub const fn new(value: u64) -> Self{
        Bitboard(value)
    }

    pub fn value(&self) -> u64{
        self.0
    }

    pub fn popultion_count(&self) -> u32{
        return self.0.count_ones();
    }

    
    pub fn check_bit(&self, index: u64) -> bool{
        return self.0 & (1 << index) != 0;
    }

    pub fn flip_bit(&mut self, index: u64){
        self.0 = self.0 ^(1 << index);
    }

    pub fn north_one(&self) -> Bitboard{
        (self  & -EIGHT_RANK) << 8
    }

    pub fn south_one(&self) -> Bitboard{
        (self & -FIRST_RANK) >> 8
    }

    pub fn west_one(&self) -> Bitboard{
        (self & -A_FILE) << 1
    }
    
    pub fn east_one(&self) -> Bitboard{
        (self & -H_FILE) >> 1
    }

    pub fn north_east_one(&self) -> Bitboard{
        (self & -H_FILE & -EIGHT_RANK) << 7
    }

    pub fn south_east_one(&self) -> Bitboard{
        (self & -H_FILE & -FIRST_RANK) >> 9
    }

    pub fn north_west_one(&self) -> Bitboard{
        (self & -A_FILE & -EIGHT_RANK) << 9
    }

    pub fn south_west_one(&self) -> Bitboard{
        (self & -A_FILE & -FIRST_RANK) >> 7
    }

    pub fn fill(&self, obstiacles: Self, direction: fn(&Bitboard) -> Bitboard) -> Bitboard{
        let mut ray = Self(self.0);

        loop{
            let mut tmp = direction(&ray);
            tmp = (tmp & obstiacles) ^ tmp;
            if (tmp | ray) == ray{
                ray = tmp | ray;
                break;
            }
            ray = tmp | ray;
        }
        return direction(&ray);
    }

    pub fn least_signicant(&self) -> Bitboard{
        if self.0 == 0 {
            return Self(0);
        }
        Self(1 << (63 - self.0.leading_zeros()))
    }

    pub fn least_signicant_index(&self) -> i32{
        if self.0 == 0 {
            return 0;
        }
        return (63 - self.0.leading_zeros()) as i32;
    }
}

impl BitAnd for Bitboard{

    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard{

    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output{
       Self(self.0 ^ rhs.0)
    }
}

impl Neg for Bitboard{
    type Output = Self;

    fn neg(self) -> Self::Output{
        Self(u64::MAX ^ self.0)
    }
}

impl BitAnd<Bitboard> for &Bitboard{

    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr<Bitboard> for &Bitboard{

    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor<Bitboard> for &Bitboard{
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output{
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Neg for &Bitboard{
    type Output = Bitboard;

    fn neg(self) -> Self::Output{
        Bitboard(u64::MAX ^ self.0)
    }
}

impl Shl<u64> for &Bitboard{
    type Output = Bitboard;

    fn shl(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u64> for &Bitboard{
    type Output = Bitboard;

    fn shr(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl Shl<u64> for Bitboard{
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u64> for Bitboard{
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl Clone for Bitboard{

    fn clone(&self) -> Bitboard {
        *self
    }
}

impl Copy for Bitboard{
}


impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut i: u64 = 64;
        while i >= 1{
            str.push(if self.check_bit(i - 1) == true {'1'} else {'0'});
            if (i-1) % 8 == 0{
                str.push('\n');
            } else {
                str.push(' ');
            }
            i = i - 1;
        }
        return write!(f, "{}", str);
    }
}


