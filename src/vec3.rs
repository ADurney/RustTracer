#[derive(Copy,Clone,Debug)]
pub struct Vec3
{
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

pub type Point3 = Vec3;

impl Vec3
{
    pub fn new (x: f32, y: f32, z: f32) -> Vec3
    {
        Vec3
        {
            x,
            y,
            z,
        }
    }

    pub fn length(&self) -> f32
    {
        f32::sqrt(self.x * self.x +
                  self.y * self.y +
                  self.z * self.z)
    }
    pub fn squared_length(&self) -> f32
    {
        self.x * self.x +
            self.y * self.y +
            self.z * self.z
    }
    pub fn make_unit_vector(v: Vec3) -> Vec3
    {
        v / v.length()
    }
    pub fn dot( v1 : &Vec3, v2 : &Vec3) -> f32
    {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }
    pub fn cross( v1 : &Vec3, v2 : &Vec3) -> Vec3
    {
        Vec3::new(v1.y * v2.z - v1.z * v2.y,
                  -(v1.x * v2.z - v1.z*v2.x),
                  v1.x * v2.y - v1.y * v2.y)
    }

}

impl std::ops::MulAssign<Vec3> for Vec3
{
    fn mul_assign(&mut self, other:Vec3)
    {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl std::ops::MulAssign<f32> for Vec3
{
    fn mul_assign(&mut self, other: f32)
    {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3
{
    fn sub_assign(&mut self, other : Vec3)
    {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;

    }
}
impl std::ops::AddAssign<Vec3> for Vec3
{
    fn add_assign(&mut self, other : Vec3)
    {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Div<f32> for Vec3
{
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3
    {
        Vec3
        {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Mul<f32> for Vec3
{
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3
    {
        Vec3
        {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Mul<Vec3> for f32
{
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}


impl std::ops::Add<Vec3> for Vec3
{
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3
{
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3
    {
        Vec3
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
