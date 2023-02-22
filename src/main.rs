use std::ops::{Add, Div, Mul, Sub};
#[derive(Debug, Clone, Copy)]

pub struct FieldPoint {
    num: u128,
    prime: u128,
}
impl FieldPoint {
    pub fn new(num: u128, prime: u128) -> Self {
        if num > prime {
            panic!("Overflow");
        } else {
            FieldPoint { num, prime }
        }
    }

    pub fn power(&self, index: u128) -> Self {
        if index == 0 {
            Self {
                num: 1,
                prime: self.prime,
            }
        } else {
            let mut aux = index.rem_euclid(self.prime - 1);
            let mut acc = 1u128;
            let mut base = self.num;

            while aux > 0 {
                if aux % 2 == 0 {
                    base = (base * base).rem_euclid(self.prime);
                    aux = aux / 2;
                } else {
                    acc = (acc * base).rem_euclid(self.prime);
                    aux = aux - 1;
                }
            }
            Self {
                num: acc,
                prime: self.prime,
            }
        }
    }

    pub fn inversion(&self) -> u128 {
        let mut t = 0i128;
        let mut r = self.prime as i128;
        let mut t1 = 1i128;
        let mut r1 = self.num as i128;

        while r1 != 0 {
            let q = r.div_euclid(r1);
            (t, t1) = (t1, t - q * t1);
            (r, r1) = (r1, r - q * r1);
        }

        if r != 1 {
            return 0;
        }
        if t < 0 {
            t = t + self.prime as i128;
        }
        t as u128
    }
}

impl Add for FieldPoint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.prime == rhs.prime {
            let num = (self.num + rhs.num).rem_euclid(rhs.prime);
            let prime = rhs.prime;
            Self { num, prime }
        } else {
            panic!("Different prime values")
        }
    }
}

impl Mul for FieldPoint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime == rhs.prime {
            let num = (self.num * rhs.num).rem_euclid(rhs.prime);
            let prime = rhs.prime;
            Self { num, prime }
        } else {
            panic!("Different prime values")
        }
    }
}

fn main() {
    let x = FieldPoint::new(5, 7);
    let y = FieldPoint::new(3, 7);
    let res_sum = x + y;
    let res_mul = x * y;
    let res_power = x.power(3);
    let res_inv = y.inversion();
    println!("res_sum: {:?}", res_sum);
    println!("res_mul: {:?}", res_mul);
    println!("res_power: {:?}", res_power);
    println!("res_inv: {:?}", res_inv);
}
