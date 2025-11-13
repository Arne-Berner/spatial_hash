use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn ceil(self: Self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }
    pub fn add(self: &Self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
    pub fn sub(self: &Self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
    pub fn div(self: &Self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
    pub fn mul(self: &Self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Vec2 + &Vec2
impl<'a> Add<&'a Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// &Vec2 + Vec2
impl<'a> Add<Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// &Vec2 + &Vec2 -> Vec2
impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

impl Add<f32> for &Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

// AddAssign for +=
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Vec2 - &Vec2
impl<'a> Sub<&'a Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// &Vec2 - Vec2
impl<'a> Sub<Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// &Vec2 - &Vec2 -> Vec2
impl<'a, 'b> Sub<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn sub(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x - rhs, self.y - rhs)
    }
}

impl Sub<f32> for &Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x - rhs, self.y - rhs)
    }
}

// SubAssign for -=
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// Vec2 / &Vec2
impl<'a> Div<&'a Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// &Vec2 / Vec2
impl<'a> Div<Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// &Vec2 / &Vec2 -> Vec2
impl<'a, 'b> Div<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn div(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl Div<f32> for &Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

// DivAssign for /=
impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Vec2) {
        self.x /= other.x;
        self.y /= other.y;
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// Vec2 * &Vec2
impl<'a> Mul<&'a Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// &Vec2 * Vec2
impl<'a> Mul<Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// &Vec2 * &Vec2 -> Vec2
impl<'a, 'b> Mul<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn mul(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<f32> for &Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

// MulAssign for *=
impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Vec2) {
        self.x *= other.x;
        self.y *= other.y;
    }
}
