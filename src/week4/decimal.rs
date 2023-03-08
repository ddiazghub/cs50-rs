use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, write};
use std::iter;
use std::ops::{Add, Mul, Sub};
use std::sync::mpsc::channel;

const CHAR_ZERO: u8 = '0' as u8;
const CHAR_NINE: u8 = '9' as u8;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
    decimal_places: usize,
    digits: Vec<u8>,
    sign: bool
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let first = input.chars().next()?;
        let sign = first == '-';
        let offset = (sign || first == '+') as usize;

        if input.len() < 1 + offset {
            return None;
        }

        let mut digits: Vec<u8> = Vec::new();
        let mut trimmed = input[offset..].trim_start_matches('0').to_string();

        if trimmed.len() == 0 || trimmed.chars().next()? == '.' {
            trimmed = format!("0{}", trimmed);
        }

        let original_length = trimmed.len();

        let mut decimal_places: usize = match trimmed.chars().position(|ch| ch == '.') {
            Some(p) => original_length - p - 1,
            None => 0
        };

        if decimal_places > 0 {
            trimmed = trimmed.trim_end_matches('0').to_string();
            decimal_places -= original_length - trimmed.len();
        }

        for ch in trimmed.chars() {
            match ch as u8 {
                CHAR_ZERO..=CHAR_NINE => digits.push(ch as u8 - CHAR_ZERO),
                46 => (),
                _ => return None
            };
        }

        if digits.is_empty() {
            digits.push(0);
        }

        if decimal_places != 0 {
            let trailing_zeros = &digits[digits.len() - decimal_places..]
                .iter()
                .rev()
                .position(|digit| *digit != 0);

            if let Some(t) = trailing_zeros {
                digits.truncate(digits.len() - *t);
            }
        }

        Some(
            Self {
                sign,
                digits,
                decimal_places
            }
        )
    }

    pub fn int_places(&self) -> usize {
        self.digits.len() - self.decimal_places
    }

    pub fn negate(mut self) -> Self {
        self.sign = !self.sign;
        self
    }

    fn compare_any<F: Fn((&u8, &u8)) -> bool>(&self, other: &Self, f: F) -> bool {
        if self.digits.len() > other.digits.len() {
            self.digits
                .iter()
                .zip(other.digits.iter().chain(iter::repeat(&0)))
                .any(f)
        } else {
            self.digits
                .iter()
                .chain(iter::repeat(&0))
                .zip(other.digits.iter())
                .any(f)
        }
    }

    fn obviously_smaller(&self, other: &Self) -> bool {
        (self.sign as u8) > (other.sign as u8) || (self.int_places() < other.int_places())
    }

    fn abs(&self) -> Self {
        let mut s = self.clone();
        s.sign = false;

        s
    }

    fn pad_left(&mut self, n: usize) {
        let mut new_digits = vec![0; n];
        new_digits.append(&mut self.digits);
        self.digits = new_digits;
    }

    fn pad_right(&mut self, n: usize) {
        self.digits.resize(self.digits.len() + n, 0);
        self.decimal_places += n;
    }

    fn digits_count(&self, other: &Self) -> (usize, usize) {
        match (self.decimal_places > other.decimal_places, self.int_places() > other.int_places()) {
            (true, true) => (self.decimal_places, self.int_places()),
            (true, false) => (self.decimal_places, other.int_places()),
            (false, false) => (other.decimal_places, other.int_places()),
            (false, true) => (other.decimal_places, self.int_places()),
        }
    }

    fn get_indexes(&self, other: &Self, start: i32, i: usize) -> (Option<usize>, Option<usize>) {
        let (rhs_len, lhs_len) = (other.digits.len(), self.digits.len());

        let offsets = if start < 0 {
            ((-1 * start) as usize, 0)
        } else {
            (0, start as usize)
        };

        match ((offsets.1..offsets.1 + rhs_len).contains(&i), (offsets.0..offsets.0 + lhs_len).contains(&i)) {
            (true, true) => (Some(rhs_len + offsets.1 - (i + 1)), Some(lhs_len + offsets.0 - (i + 1))),
            (false, true) => (None, Some(lhs_len + offsets.0 - (i + 1))),
            (false, false) => (None, None),
            (true, false) => (Some(rhs_len + offsets.1 - (i + 1)), None),
        }
    }

    fn operate_digits<F: Fn(&Self, &Self, &mut Vec<i32>, i32) -> ()>(self, other: Self, f: F) -> Self {
        let (decimal_places, int_places) = self.digits_count(&other);

        let mut digits = vec![0; decimal_places + int_places];
        let start_offset = self.decimal_places as i32 - other.decimal_places as i32;

        f(&self, &other, &mut digits, start_offset);

        let temp = Self {
            sign: self.sign,
            decimal_places,
            digits: digits
                .into_iter()
                .rev()
                .map(|v| v as u8)
                .collect()
        };

        Self::try_from(&temp.to_string()).unwrap()
    }

    pub fn digit_wise_operate<F: Fn(&Decimal, &Decimal, &mut Vec<i32>, i32, usize)>(&self, other: &Self, digits: &mut Vec<i32>, start_offset: i32, operation: F) {
        for i in 0..digits.len() {
            operation(self, other, digits, start_offset, i);
        }
    }

    fn add1(&self, other: &Self, digits: &mut Vec<i32>, start_offset: i32) {
        if self.sign ^ other.sign {
            let offset = if start_offset < 0 {
                (-1 * start_offset) as usize
            } else {
                0
            };

            let len = self.digits.len();

            for i in 0..digits.len() {
                if (offset..offset + self.digits.len()).contains(&i) {
                    digits[i] = self.digits[len + offset - (i + 1)] as i32;
                }
            }

            self.digit_wise_substract(other, digits, start_offset);
        } else {
            self.digit_wise_add(other, digits, start_offset);
        }
    }

    fn add_digits(self, other: Self) -> Self {
        self.operate_digits(other, Self::add1)
    }

    pub fn digit_wise_add(&self, other: &Self, digits: &mut Vec<i32>, start_offset: i32) {
        self.digit_wise_operate(other, digits, start_offset, |number, other, digits, start, i| {
            let (j, k) = self.get_indexes(other, start, i);

            if let Some(n) = j {
                digits[i] += other.digits[n] as i32;
            }

            if let Some(n) = k {
                digits[i] += number.digits[n] as i32;
            }

            if digits[i] >= 10 {
                if i == digits.len() - 1 {
                    digits.push(0);
                }

                digits[i] = digits[i] - 10;
                digits[i + 1] += 1;
            }
        });
    }

    pub fn digit_wise_substract(&self, other: &Self, digits: &mut Vec<i32>, start_offset: i32) {
        self.digit_wise_operate(other, digits, start_offset, |number, other, digits, start, i| {
            let (j, k) = self.get_indexes(other, start, i);
            if let Some(n) = j {
                digits[i] -= other.digits[n] as i32;
            }

            if digits[i] < 0 {
                digits[i] += 10;
                let mut i2 = i + 1;

                while digits[i2] == 0 {
                    digits[i2] = 9;
                    i2 += 1;
                }

                digits[i2] -= 1;
            }
        });
    }

    fn multiply_digits(self, other: Self) -> Self {
        self.operate_digits(other, Self::digit_wise_multiply)
    }

    pub fn digit_wise_multiply(&self, other: &Self, digits: &mut Vec<i32>, start_offset: i32) {
        self.digit_wise_operate(other, digits, start_offset, |number, other, digits, start, i| {
            for (i2, j) in (0..number.digits.len()).rev().enumerate() {
                if i + i2 == digits.len() {
                    digits.push(0);
                }

                digits[i + i2] += other.digits[j] as i32 * number.digits[number.digits.len() - (i + 1)] as i32;
            }

            if digits[i] >= 10 {
                digits[i + 1] += digits[i] / 10;
                digits[i] = digits[i] % 10;
            }
        });
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        if self.sign {
            string.push('-')
        }

        for (i, digit) in self.digits.iter().enumerate() {
            if self.digits.len() - i == self.decimal_places {
                string.push('.');
            }

            string.push((CHAR_ZERO + *digit) as char);
        }

        write!(f, "{}", string)
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let maybe_equal = self.sign == other.sign && self.decimal_places == other.decimal_places;

        maybe_equal && !self.compare_any(other, |(d1, d2)| *d1 != *d2)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = if self.eq(other) {
            Ordering::Equal
        } else if self.lt(other) {
            Ordering::Less
        } else {
            Ordering::Greater
        };

        Some(ordering)
    }

    fn le(&self, other: &Self) -> bool {
        self.obviously_smaller(other) || if self.sign {
            !self.compare_any(other, |(d1, d2)| *d1 > *d2)
        } else {
            !self.compare_any(other, |(d1, d2)| *d1 > *d2)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.obviously_smaller(other) || if self.sign {
            self.compare_any(other, |(d1, d2)| *d1 > *d2)
        } else {
            self.compare_any(other, |(d1, d2)| *d1 < *d2)
        }
    }

    fn ge(&self, other: &Self) -> bool {
        other.obviously_smaller(self) || if self.sign {
            !self.compare_any(other, |(d1, d2)| *d1 > *d2)
        } else {
            !self.compare_any(other, |(d1, d2)| *d1 < *d2)
        }
    }

    fn gt(&self, other: &Self) -> bool {
        other.obviously_smaller(self) || if self.sign {
            self.compare_any(other, |(d1, d2)| {
                *d1 < *d2
            })
        } else {
            self.compare_any(other, |(d1, d2)| {
                *d1 > *d2
            })
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.sign ^ rhs.sign && self.abs() > rhs.abs() {
            self.add_digits(rhs)
        } else {
            rhs.add_digits(self)
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.negate())
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        let decimal_places = self.decimal_places + rhs.decimal_places;
        let int_places = self.int_places() + rhs.int_places();

        self.pad_right(decimal_places - self.decimal_places);
        self.pad_left(int_places - self.int_places());
        rhs.pad_right(decimal_places - rhs.decimal_places);
        rhs.pad_left(int_places - rhs.int_places());

        let sign = self.sign ^ rhs.sign;
        let mut result = self.multiply_digits(rhs);
        result.decimal_places = decimal_places;
        result.sign = sign;

        Self::try_from(&result.to_string()).unwrap()
    }
}

pub fn main() {
    let n1 = Decimal::try_from("2.1").unwrap();
    let n2 = Decimal::try_from("1.0").unwrap();
    
    println!("{} * {} => {}", n1.clone(), n2.clone(), n1 * n2);
}
