mod vec3;
mod ray;

use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;



pub struct Sphere
{
    center : Vec3,
    radius : f32,
}

impl Sphere
{
    pub fn new (center : Vec3, radius : f32) ->Sphere
    {
        Sphere{
            center,
            radius,
        }
    }
    pub fn hit(&self, ray : Ray) -> bool
    {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(),&ray.direction());
        let b = 2.0 * Vec3::dot(&oc,&ray.direction());
        let c = Vec3::dot(&oc,&oc) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        (discriminant > 0.0) as bool

    }
}


type Colour = Vec3;

fn write_colour(pixel_colour: Colour)
{
    let ir = (255.00*pixel_colour.x) as i32;
    let ig = (255.00*pixel_colour.y) as i32;
    let ib = (255.00*pixel_colour.z) as i32;
    print!("{} {} {}\n", ir, ig, ib);

}

fn ray_colour(ray: Ray) -> Colour
{
    let sphr = Sphere::new(Point3::new(0.0,0.0,1.0),0.5);
    let collision = sphr.hit(ray);

    let return_colour =
        if collision {
            Colour::new(1.0,0.0,0.0)
        } else {
            let unit_direction = Vec3::make_unit_vector(ray.direction()) as Vec3;
            let t = 0.5 * (unit_direction.y + 1.0) as f32;
            ((1.0-t)*Colour::new(1.0,1.0,1.0) + t * Colour::new(0.0,1.0,1.0)) as Colour
        };
        return_colour

    

}


fn main() 
{
    //Image
    let aspect_ratio = 16.0/8.0 as f32;
    let image_width = 400;
    let image_height_f =(image_width as f32)/aspect_ratio;
    let image_height =  image_height_f.trunc() as i32;


    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width,0.0,0.0);
    let vertical = Vec3::new(0.0,viewport_height,0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,focal_length);

    
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev()
    {
        for i in 0..image_width
        {
            let u = ((i as f32) / (image_width as f32) -1.0) as f32;
            let v = ((j as f32) / (image_height as f32) -1.0) as f32;
            let r = Ray::new(origin, lower_left_corner + u* horizontal + v*vertical - origin);
            let pixel_colour = ray_colour(r);
            write_colour(pixel_colour);
        }
    }
    
}

