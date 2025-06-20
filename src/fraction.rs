use std::fmt::Debug;

use facet_macros::Facet;
#[derive(PartialEq, Eq, Clone, Copy,Facet)]
pub struct fraction {
    numerator: u32,

    denominator: u32,
    positive: bool,
}
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
impl fraction {
    fn simplify(&mut self) {
        let divisor = gcd(self.numerator, self.denominator);
        self.numerator /= divisor;
        self.denominator /= divisor;
    }
    pub fn new(numerator: i32, denominator: i32) -> fraction {
        assert!(denominator != 0, "Denominator cannot be zero");
        let positive = numerator * denominator > 0;
        let mut frac = fraction {
            numerator: numerator.abs() as u32,
            denominator: denominator.abs() as u32,
            positive,
        };
        frac.simplify();
        frac
    }
    pub fn to_num<T>(&self) -> T
    where
        T: num_traits::NumCast + std::ops::Div<Output = T> + std::ops::Mul<Output = T> + Copy,
    {
        if self.denominator == 0 {
            panic!("Cannot convert fraction with zero denominator to number");
        }
        let sign: T = if self.positive {
            T::from(1).unwrap()
        } else {
            T::from(-1).unwrap()
        };
        let num = T::from(self.numerator).unwrap();
        let den = T::from(self.denominator).unwrap();
        sign * (num / den)
    }
}
impl std::ops::Add for fraction {
    type Output = fraction;

    fn add(self, other: fraction) -> fraction {
        // 如果操作数符号不同，则用减法
        if self.positive != other.positive {
            self - (-other)
        } else {
            // 同号直接加
            let mut result = fraction {
            numerator: self.numerator * other.denominator + other.numerator * self.denominator,
            denominator: self.denominator * other.denominator,
            positive: self.positive,
            };
            result.simplify();
            result
        }
    }
}
impl std::ops::Sub for fraction {
    type Output = fraction;

    fn sub(self, other: fraction) -> fraction {
        let numerator=self.numerator as i32 * other.denominator as i32 - other.numerator as i32* self.denominator as i32;
        let mut result = fraction {
            numerator:numerator.abs() as u32,
            denominator: self.denominator * other.denominator,
            positive: self.positive&&numerator>0,
        };
        result.simplify();
        result
    }
}
impl std::ops::Mul for fraction {
    type Output = fraction;

    fn mul(self, other: fraction) -> fraction {
        let mut result = fraction {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
            positive: self.positive == other.positive,
        };
        result.simplify();
        result
    }
}
impl std::ops::Div for fraction {
    type Output = fraction;

    fn div(self, other: fraction) -> fraction {
        let mut result = fraction {
            numerator: self.numerator * other.denominator,
            denominator: self.denominator * other.numerator,
            positive: self.positive == other.positive,
        };
        result.simplify();
        result
    }
}
impl std::ops::Neg for fraction {
    type Output = fraction;

    fn neg(self) -> fraction {
        fraction {
            numerator: self.numerator,
            denominator: self.denominator,
            positive: !self.positive,
        }
    }
}
impl <T>  From<T> for fraction
where
    T: num_traits::NumCast + Copy,
{
    fn from(value: T) -> fraction {
        let float_val: f64 = num_traits::cast::<T, f64>(value).expect("Unsupported type for fraction conversion");
        //获取小数位数
        let decimal_places = if float_val.fract() == 0.0 {
            0
        } else {
            let mut count = 0;
            let mut temp = float_val;
            while temp.fract() > 0.1 && count < 10 {
                temp *= 10.0;
                count += 1;
            }
            count
        };
        let denom = 10u32.pow(decimal_places as u32);
        let num = (float_val * denom as f64).round();
        fraction::new(num.round() as i32, denom as i32)
    }
}
impl Debug for fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.positive { "" } else { "-" };
        write!(f, "{}{}/{}", sign, self.numerator, self.denominator)
    }
}