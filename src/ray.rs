use raylib::prelude::*;

use crate::vec3::Vec3;

const CANVAS_HEIGHT : i32 = 720;
const CANVAS_WIDTH  : i32 = 1280;


const PIXEL_SIZE : i32 = 5;

const IMAGE_WIDTH : i32 = 256;

const IMAGE_HEIGHT : i32 = 144;

const PIXEL_COUNT : usize = (IMAGE_WIDTH*IMAGE_HEIGHT) as usize;

//single pixel struct
#[derive(Clone,Copy)]
struct Pixel{
    pos_x:i32,
    pos_y:i32,
    p_r : u8,
    p_g : u8,
    p_b : u8,
    p_color : Color,
}

pub struct Ray{
    
    pixels : [Pixel;PIXEL_COUNT],
    IMAGE_WIDTH : u32 , 
    IMAGE_HEIGHT : u32,
}

impl Ray{
    
    
    //init ray
    pub fn Init_Ray() -> Self{
	//check the sizes
	println!("Image width : {}",IMAGE_WIDTH);
	println!("Image height : {}",IMAGE_HEIGHT);
	
	//vec3
	//let mut v3  = Vec3::Init();

	let aspect_ratio : f64 = 16.0/9.0;
	let image_width : i32 = IMAGE_WIDTH;

	//calc image height
	//let image_height : i32 = (image_width as f64/aspect_ratio) as i32;
	let image_height : i32 = IMAGE_HEIGHT;
	
	//camera
	let focal_length : f64 = 1.0;
	let viewport_height : f64 = 2.0;
	let viewport_width : f64 = viewport_height*(image_width as f64 /image_height as f64);
	println!("Viewport_width: {}",viewport_width);
	
	let camera_centre : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);

	//vec across: they give full width or hor and vert value of the canvas
	let viewport_u : Vec3 = Vec3::Init_Vec(viewport_width,0.0,0.0);
	let viewport_v : Vec3 = Vec3::Init_Vec(0.0,-viewport_height,0.0);
	println!("\nViewport_u n v");
	viewport_u.print_val();
	viewport_v.print_val();

	//delta vecs:these are small vals used for moving either left or right in viewport
	let mut pixel_delta_u : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);
	pixel_delta_u.Div_Vec_Double(viewport_u.clone(),image_width as f64);
	
	let mut pixel_delta_v : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);
	pixel_delta_v.Div_Vec_Double(viewport_v.clone(),image_height as f64);
	println!("\npixel_delta_u n v");
	pixel_delta_u.print_val();
	pixel_delta_v.print_val();
	//upper left pixel proof
	let mut viewport_upper_left : Vec3 = camera_centre.clone();

	//sub vec3's
	let focal_len_vec : Vec3 = Vec3::Init_Vec(0.0,0.0,focal_length);
	viewport_upper_left.Sub_Self_Vec(focal_len_vec);
	
	viewport_upper_left.Sub_Self_Vec_ul(viewport_u.clone(),2.0);
	
	viewport_upper_left.Sub_Self_Vec_ul(viewport_v.clone(),2.0);

	println!("\nUpper left vec");
	viewport_upper_left.print_val();

	//calculate centre of first pixel
	let mut pixel_00_loc : Vec3 = viewport_upper_left.clone();
	let temp_vec_v : Vec3 = pixel_delta_v.clone();
	let temp_vec_u : Vec3 = pixel_delta_u.clone();
	//set  temp vec for 0.5*(vec3+vec3)
	let mut temp_vec : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);
	temp_vec.Sum_Vec_Vec(temp_vec_v.clone(),temp_vec_u.clone());
	println!("\nsum before mul 0.5");
	temp_vec.print_val();
	
	temp_vec.Mul_Self_Vec(0.5);
	println!("\n after mul 0.5");
	temp_vec.print_val();
	
	//for final centre pixel location
	pixel_00_loc.Sum_Self_Vec(temp_vec.clone());
	println!("\nPixel 00 loc!");
	pixel_00_loc.print_val();
	
	let mut count : usize = 0;
	let mut pixel = Pixel{pos_x: 0,pos_y: 0,p_r:0,p_g:0,p_b:0,p_color:Color::new(255,0,0,1)};

	let mut pixel_arr = [pixel;PIXEL_COUNT];

	//init some vec3 for color calculation
	let mut pixel_centre : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);
	
	let mut temp_pixel_delta_u : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);
	let mut temp_pixel_delta_v : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);

	let mut ray_direction : Vec3 = Vec3::Init_Vec(0.0,0.0,0.0);

	//variables for the linear blend
	let mut temp_color_3 : Vec3  = Vec3::Init_Vec(0.0,0.0,0.0);

	let blend_col1 : Vec3 = Vec3::Init_Vec(0.5,0.7,1.0);
	let blend_col2 : Vec3 = Vec3::Init_Vec(1.0,1.0,1.0);

	println!("\nStart loop !! \n");
	//init pixel pos
	for r in 0..image_height{
	    //println!("Scanlines remaining : {}",r);
	    for c in 0..image_width{
		//setting the pos
		pixel_arr[count].pos_x = c;
		pixel_arr[count].pos_y = r;
		
		//1. set the centre of current pixel

		temp_pixel_delta_u.Mul_Val_Vec(pixel_delta_u.clone(),c as f64);
		temp_pixel_delta_v.Mul_Val_Vec(pixel_delta_v.clone(),r as f64);
		
		pixel_centre = pixel_00_loc.clone();
		pixel_centre.Sum_Self_Vec(temp_pixel_delta_u.clone());
		pixel_centre.Sum_Self_Vec(temp_pixel_delta_v.clone());

		//ray direction
		ray_direction.Sub_Vec_Vec(pixel_centre.clone(),camera_centre.clone());

		//test-------
		temp_color_3 = temp_color_3.Calculate_Color(ray_direction.clone(),blend_col1.clone(),
							    blend_col2.clone(),r);
		//color got
		
		//convert the [0,1] to color
		temp_color_3.Convert_To_Color();
		//temp_color_3.Print_Color();

		
		if c<256 && r<1{
		    println!("r: {} c : {}",r,c);
		    temp_color_3.Print_Color();
		} 
		// ---------Old way! ______set color
		// let temp_r : f32 = (r as f32)/(IMAGE_HEIGHT-1) as f32;
		// let temp_g : f32 = (c as f32)/(IMAGE_WIDTH-1) as f32;
		// let temp_b : f32 = 0.0;

		// //println!("tr: {} tg: {} tb: {}",temp_r,temp_g,temp_b);
		
		// let i_r : i32 = (255.999 * temp_r) as i32;
		// let i_g : i32 = (255.999 * temp_g) as i32;
		// let i_b : i32 = (255.999 * temp_b) as i32;

		//println!("ir: {} ig: {} ib: {}",i_r,i_g,i_b);
		
		pixel_arr[count].p_r = temp_color_3.Get_R() as u8;
		pixel_arr[count].p_g = temp_color_3.Get_G() as u8;
		pixel_arr[count].p_b = temp_color_3.Get_B() as u8;

		let color = Color::new(pixel_arr[count].p_r,pixel_arr[count].p_g,pixel_arr[count].p_b,255);
		pixel_arr[count].p_color = color;
		
		count+=1;
	    }
	}
	println!("Done! total count : {}",count);
	Ray {pixels : pixel_arr,IMAGE_WIDTH : IMAGE_WIDTH as u32,IMAGE_HEIGHT : IMAGE_HEIGHT as u32}
    }
    
    pub fn Draw_Ray(&self,rl: &mut RaylibHandle,thread: &RaylibThread){
	
	let mut d = rl.begin_drawing(&thread);

	d.clear_background(Color::WHITE);
	d.draw_text("yeay",100,100,10,Color::RED);
	
	//draw pixel
	for pixel in self.pixels{
	    //println!("\n X :{} Y: {}",pixel.pos_x,pixel.pos_y);
	    d.draw_rectangle(pixel.pos_x*PIXEL_SIZE,pixel.pos_y*PIXEL_SIZE,
			     PIXEL_SIZE,PIXEL_SIZE,pixel.p_color);
	}
	//draw grids
	for col in 0..200{
	    //vert
	    // d.draw_line(col*PIXEL_SIZE as i32 ,0,col*PIXEL_SIZE as i32 ,1000,Color::BLACK);
	    // //hor
	    // d.draw_line(0,col*PIXEL_SIZE as i32 ,1000,col*PIXEL_SIZE as i32 ,Color::BLACK);
	}
	
    }

}
