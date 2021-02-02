use std::fmt;
use std::ops;

/// Vector2 for represent 2d coord.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    /// Constructor of Vector2
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }
    /// Getter of x
    pub fn get_x(&self) -> i32 {
        self.x
    }
    /// Getter of y
    pub fn get_y(&self) -> i32 {
        self.y
    }
    /// Setter of Vector2
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
impl fmt::Display for Vector2{
    ///Displays as (x,y) form.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})",self.x,self.y)

    }
}
impl ops::Add for Vector2 {
    type Output = Self;
    /// Element-wise add
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;
    /// Element-wise sub
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl ops::Mul<Vector2> for Vector2 {
    type Output = i32;
    /// Inner product
    fn mul(self, other: Vector2) -> i32 {
            self.x * other.x+ self.y * other.y
    }
}

impl ops::Mul<i32> for Vector2 {
    type Output = Self;
    ///Scala multiplication
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::MulAssign<i32> for Vector2 {
    fn mul_assign(&mut self, other: i32) {
        *self =Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::Div<i32> for Vector2 {
    type Output = Self;
    /// Scala division
    fn div(self, other: i32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl ops::DivAssign<i32> for Vector2 {
    fn div_assign(&mut self, other: i32) {
        *self =Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn new() {
        assert_eq!(Vector2::new(3,4), Vector2{x:3,y:4});
}
#[test]
fn get() {
    let v=Vector2::new(-3,4);
    assert_eq!(v.get_x(), -3);
    assert_eq!(v.get_y(), 4);
}
#[test]
fn set() {
    let mut v=Vector2::new(-3,4);
    assert_eq!(v, Vector2{x:-3,y:4});
    v.set(13,-6);
    assert_eq!(v, Vector2{x:13,y:-6});
}
#[test]
fn display() {
    let v=Vector2::new(-4, -5);
    assert_eq!(format!("{}",v), String::from("(-4,-5)"))
}
#[test]
fn add() {
    let mut v=Vector2::new(5, 4);
    let w=Vector2::new(6, -3);
    assert_eq!(v+w,Vector2{x:11,y:1});
    assert_eq!(v,Vector2{x:5,y:4});
    v+=w;
    assert_eq!(v,Vector2{x:11,y:1});
}
#[test]
fn sub() {
    let mut v=Vector2::new(5, 4);
    let w=Vector2::new(6, -3);
    assert_eq!(v-w,Vector2{x:-1,y:7});
    assert_eq!(v,Vector2{x:5,y:4});
    v-=w;
    assert_eq!(v,Vector2{x:-1,y:7});
}
#[test]
fn inner_product() {
    let v=Vector2::new(3, 4);
    let w = Vector2::new(3,4);
    assert_eq!(v*w,25);
}
#[test]
fn mul() {
    let mut v=Vector2::new(5, 4);
    assert_eq!(v*3,Vector2{x:15,y:12});
    assert_eq!(v,Vector2{x:5,y:4});
    v*=3;
    assert_eq!(v,Vector2{x:15,y:12});
}
#[test]
fn div() {
    let mut v=Vector2::new(5, 4);
    assert_eq!(v/3,Vector2{x:1,y:1});
    assert_eq!(v,Vector2{x:5,y:4});
    v/=3;
    assert_eq!(v,Vector2{x:1,y:1});
}
}