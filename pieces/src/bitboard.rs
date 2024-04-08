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
        if self.0 == 0 {
            return *self;
        }
        return (self  & -EIGHT_RANK) << 8
    }

    pub fn south_one(&self) -> Bitboard{
        if self.0 == 0 {
            return *self;
        }
        return (self & -FIRST_RANK) >> 8
    }

    pub fn west_one(&self) -> Bitboard{
        if self.0 == 0 {
            return *self;
        }
        return (self & -A_FILE) << 1
    }
    
    pub fn east_one(&self) -> Bitboard{
        if self.0 == 0 {
            return *self;
        }
        return (self & -H_FILE) >> 1
    }

    pub fn north_east_one(&self) -> Bitboard{
        return self.north_one().east_one();
    }

    pub fn south_east_one(&self) -> Bitboard{
        return self.south_one().east_one();
    }

    pub fn north_west_one(&self) -> Bitboard{
        return self.north_one().west_one();
    }

    pub fn south_west_one(&self) -> Bitboard{
        return self.south_one().west_one();
    }

    pub fn fill(&self, obstiacles: Self, direction: fn(&Bitboard) -> Bitboard) -> Bitboard{
        if self.0 == 0{
            return *self;
        }
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
        return direction(&ray) | *self;
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
        return Self(self.0 & rhs.0);
    }
}

impl BitOr for Bitboard{

    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        return Self(self.0 | rhs.0);
    }
}

impl BitXor for Bitboard{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output{
       return Self(self.0 ^ rhs.0);
    }
}

impl Neg for Bitboard{
    type Output = Self;

    fn neg(self) -> Self::Output{
        return Self(u64::MAX ^ self.0);
    }
}

impl BitAnd<Bitboard> for &Bitboard{

    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        return Bitboard(self.0 & rhs.0);
    }
}

impl BitOr<Bitboard> for &Bitboard{

    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        return Bitboard(self.0 | rhs.0);
    }
}

impl BitXor<Bitboard> for &Bitboard{
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output{
        return Bitboard(self.0 ^ rhs.0);
    }
}

impl Neg for &Bitboard{
    type Output = Bitboard;

    fn neg(self) -> Self::Output{
        return Bitboard(u64::MAX ^ self.0);
    }
}

impl Shl<u64> for &Bitboard{
    type Output = Bitboard;

    fn shl(self, rhs: u64) -> Self::Output {
        return Bitboard(self.0 << rhs);
    }
}

impl Shr<u64> for &Bitboard{
    type Output = Bitboard;

    fn shr(self, rhs: u64) -> Self::Output {
        return Bitboard(self.0 >> rhs);
    }
}

impl Shl<u64> for Bitboard{
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        return Bitboard(self.0 << rhs);
    }
}

impl Shr<u64> for Bitboard{
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        return Bitboard(self.0 >> rhs);
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, rhs: &Self) -> bool {
        return self.0 == rhs.0;
    }
}

impl Clone for Bitboard{

    fn clone(&self) -> Bitboard {
        return *self;
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

#[cfg(test)]
mod test{
    use super::Bitboard;

    #[test]
    fn flip_bit(){
        let mut board = Bitboard(0);
        board.flip_bit(0);
        assert_eq!(board.value(), 1);
    }

    #[test]
    fn check_bit(){
        let mut board = Bitboard(0);
        board.flip_bit(5);
        assert!(board.check_bit(5));
    }

    #[test]
    fn population_count(){
        let mut board = Bitboard(0);
        board.flip_bit(0);
        assert_eq!(board.popultion_count(), 1);
        board.flip_bit(1);
        assert_eq!(board.popultion_count(), 2);
    }

    #[test]
    fn least_significant(){
        let mut board = Bitboard(0);
        board.flip_bit(0);
        board.flip_bit(1);
        let least = board.least_signicant();
        assert_ne!(least.value(), board.value());
        board.flip_bit(0);
        assert_eq!(least.value(), board.value());
    }

    #[test]
    fn least_significant_index(){
        let mut board = Bitboard(0);
        board.flip_bit(0);
        board.flip_bit(5);
        let mut least = board.least_signicant_index();
        assert_eq!(least, 5);
        board.flip_bit(5);
        least = board.least_signicant_index();
        assert_eq!(least, 0);
    }

    #[test]
    fn north_one(){
        let mut board = Bitboard(0);
        board = board.north_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(0);
        board = board.north_one();
        assert!(board.check_bit(8));
        board.flip_bit(8);
        board.flip_bit(63);
        board = board.north_one();
        assert_eq!(board.value(), 0);
    }
    #[test]
    fn south_one(){
        let mut board = Bitboard(0);
        board = board.south_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(63);
        board = board.south_one();
        assert!(board.check_bit(55));
        board.flip_bit(55);
        board.flip_bit(0);
        board = board.south_one();
        assert_eq!(board.value(), 0);
    }

    #[test]
    fn west_one(){
        let mut board = Bitboard(0);
        board = board.west_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(0);
        board = board.west_one();
        assert!(board.check_bit(1));
        board.flip_bit(1);
        board.flip_bit(7);
        board = board.west_one();
        assert_eq!(board.value(), 0);
    }

    #[test]
    fn east_one(){
        let mut board = Bitboard(0);
        board = board.east_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(7);
        board = board.east_one();
        assert!(board.check_bit(6));
        board.flip_bit(6);
        board.flip_bit(0);
        board = board.east_one();
        assert_eq!(board.value(), 0);
    }

    #[test]
    fn north_west_one(){
        let mut board = Bitboard(0);
        board = board.north_west_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(0);
        board = board.north_west_one();
        assert!(board.check_bit(9));
        board.flip_bit(9);
        board.flip_bit(63);
        board = board.north_west_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(62);
        board = board.north_west_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(55);
        board = board.north_west_one();
        assert_eq!(board.value(), 0);
    }

    #[test]
    fn north_east_one(){
        let mut board = Bitboard(0);
        board = board.north_east_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(7);
        board = board.north_east_one();
        assert!(board.check_bit(14));
        board.flip_bit(14);
        board.flip_bit(56);
        println!("{}", board);
        board = board.north_east_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(57);
        board = board.north_east_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(48);
        board = board.north_east_one();
        assert_eq!(board.value(), 0);
    }

    #[test]
    fn south_west_one(){
        let mut board = Bitboard(0);
        board = board.south_west_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(56);
        board = board.south_west_one();
        assert!(board.check_bit(49));
        println!("{}", board);
        board.flip_bit(49);
        board.flip_bit(7);
        board = board.south_west_one();
        println!("{}", board);
        assert_eq!(board.value(), 0);
        board.flip_bit(6);
        board = board.south_west_one();
        println!("{}", board);
        assert_eq!(board.value(), 0);
        board.flip_bit(63);
        board = board.south_west_one();
        println!("{}", board);
        assert_eq!(board.value(), 0);
    }

    #[test]
    fn south_east_one(){
        let mut board = Bitboard(0);
        board = board.south_east_one();
        assert_eq!(board.value(), 0);
        board.flip_bit(63);
        board = board.south_east_one();
        assert!(board.check_bit(54));
        println!("{}", board);
        board.flip_bit(54);
        board.flip_bit(0);
        board = board.south_east_one();
        println!("{}", board);
        assert_eq!(board.value(), 0);
        board.flip_bit(1);
        board = board.south_east_one();
        println!("{}", board);
        assert_eq!(board.value(), 0);
        board.flip_bit(8);
        board = board.south_east_one();
        println!("{}", board);
        assert_eq!(board.value(), 0);
    }

    #[test]
    fn fill(){
        let mut board = Bitboard(1);
        let mut obsticales = Bitboard(0);
        let mut filled = board.fill(obsticales, Bitboard::north_one);
        assert_eq!(filled.popultion_count(), 8);
        filled = board.fill(obsticales, Bitboard::west_one);
        assert_eq!(filled.popultion_count(), 8);
        board = Bitboard(0);
        board.flip_bit(63);
        filled = board.fill(obsticales, Bitboard::south_one);
        assert_eq!(filled.popultion_count(), 8);
        filled = board.fill(obsticales, Bitboard::east_one);
        assert_eq!(filled.popultion_count(), 8);
        filled = board.fill(obsticales, Bitboard::south_east_one);
        filled = filled.fill(obsticales, Bitboard::east_one);
        assert_eq!(filled.popultion_count(), 36);
        obsticales.flip_bit(62);
        filled = board.fill(obsticales, Bitboard::east_one);
        print!("{}", filled);
        assert_eq!(filled.popultion_count(), 2);

    }
}


