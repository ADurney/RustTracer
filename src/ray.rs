
use crate::vec3::Vec3;
use crate::vec3::Point3;

#[derive(Copy,Clone,Debug)]
pub struct Ray
{
    origin : Point3,
    direction : Vec3,
}

impl Ray
{
    pub fn new( origin: Point3, direction : Vec3) -> Ray
    {
        Ray {origin, direction}
    }

    pub fn at(self, distance: f32) -> Point3
    {
        self.origin + distance * self.direction
    }
    pub fn direction(&self) -> Vec3{ self.direction}
    pub fn origin(&self) -> Vec3{ self.origin}

}