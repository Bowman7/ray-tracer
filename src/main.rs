use raylib::prelude::*;

mod ray;
mod vec3;
use ray::Ray;

fn main(){
    let (mut rl,thread)  = raylib::init()
	.size(1280,720)
	.title("Raytracer in a weekend")
	.build();

    //init raylib
    let mut ray_main = Ray::Init_Ray();
    
    while !rl.window_should_close(){
	
	ray_main.Draw_Ray(&mut rl,&thread);
    }
}
