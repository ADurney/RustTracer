#[derive(Copy,Clone,Debug)]
pub struct Vec3
{
    e : [f32; 3],
}


impl Vec3
{
    pub fn new (e0: f32, e1: f32, e2: f32) -> Vec3
    {
        Vec3
        {
            e:[e0,e1,e2]
        }
    }
    pub fn x(&self) -> f32
    {
        self.e[0]
    }
    pub fn y(&self) -> f32
    {
        self.e[1]
    }
    pub fn z(&self) -> f32
    {
        self.e[2]
    }
    pub fn r(&self) -> f32
    {
        self.e[0]
    }
    pub fn g(&self) -> f32
    {
        self.e[1]
    }
    pub fn b(&self) -> f32
    {
        self.e[2]
    }
    pub fn length(&self) -> f32
    {
        f32::sqrt(self.e[0] * self.e[0] +
                  self.e[1] * self.e[1] +
                  self.e[2] * self.e[2])
    }
    pub fn squared_length(&self) -> f32
    {
        self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2]
    }
    pub fn make_unit_vector(v: Vec3) -> Vec3
    {
        v / v.length()
    }
    pub fn dot( v1 : &Vec3, v2 : &Vec3) -> f32
    {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }
    pub fn cross( v1 : &Vec3, v2 : &Vec3) -> Vec3
    {
        Vec3::new(v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                  -(v1.e[0] * v2.e[2] - v1.e[2]*v2.e[0]),
                  v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0])
    }

}

impl std::ops::MulAssign<Vec3> for Vec3
{
    fn mul_assign(&mut self, other:Vec3)
    {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl std::ops::MulAssign<f32> for Vec3
{
    fn mul_assign(&mut self, other: f32)
    {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}


pub struct Sphere
{
    center : Vec3,
    radius : f32,
}

impl Sphere
{
    pub fn new (center : Vec3, radius : f32)
    {
        center;
        radius;
    }
}


fn main() 
{
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev()
    {
        for i in 0..nx
        {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b :f32 = 0.2;
            let ir = (255.00*r) as i32;
            let ig = (255.00*g) as i32;
            let ib = (255.00*b) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}

