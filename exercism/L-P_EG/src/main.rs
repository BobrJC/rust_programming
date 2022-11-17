// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::any;

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
   
    for (i, el) in iter.enumerate() {
        if i % 2 == 1 {
            new_iter
        }
    }
    
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        (self.0.pow(2)+self.1.pow(2)).pow(0.5)
    }
}

pub fn main() {

}