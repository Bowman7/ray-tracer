#[derive(Clone,Debug)]
pub struct Vec3{
    v1 : f64,
    v2 : f64,
    v3 : f64,
    vec_r : i64,
    vec_g : i64,
    vec_b : i64,
}

impl Vec3{
    
    pub fn Init_Vec(v1: f64, v2: f64 ,v3: f64)->Self{
	Vec3{v1 : v1,v2: v2, v3: v3, vec_r: 0,vec_g: 0,vec_b: 0}
    }

    pub fn print_val(&self){
	println!("Val: {} {} {}",self.v1,self.v2,self.v3);
    }
    pub fn Div_Vec_Double(&mut self,v : Vec3, val: f64){
	self.v1 =v.v1 * (1.0/val);
	self.v2 =v.v2 * (1.0/val);
	self.v3 =v.v3 * (1.0/val);
	
    }
    //sub self to vec3
    pub fn Sub_Self_Vec(&mut self, p_vec : Vec3){
	self.v1 = self.v1  - p_vec.v1;
	self.v2 = self.v2  - p_vec.v2;
	self.v3 = self.v3  - p_vec.v3;
    }
    //sub a vec3 to vec3
    pub fn Sub_Vec_Vec(&mut self, p_v1 : Vec3, p_v2 : Vec3){
	self.v1 = p_v1.v1  - p_v2.v1;
	self.v2 = p_v1.v2  - p_v2.v2;
	self.v3 = p_v1.v3  - p_v2.v3;
    }
    //sub vec3 to vec3 for upper left pixel
    pub fn Sub_Self_Vec_ul(&mut self, p_vec : Vec3, val: f64){
	let t1 : f64 = p_vec.v1/val;
	let t2 : f64 = p_vec.v2/val;
	let t3 : f64 = p_vec.v3/val;
	
	self.v1 = self.v1  - t1;
	self.v2 = self.v2  - t2;
	self.v3 = self.v3  - t3;
    }
    //sum two vec3
    pub fn Sum_Vec_Vec(&mut self, v_1 : Vec3 , v_2 : Vec3){
	self.v1 = v_1.v1 + v_2.v1;
	self.v2 = v_1.v2 + v_2.v2;
	self.v3 = v_1.v3 + v_2.v3;
    }
    //sum self to vec3
    pub fn Sum_Self_Vec(&mut self,v_1 : Vec3){
	self.v1 = self.v1 + v_1.v1;
	self.v2 = self.v2 + v_1.v2;
	self.v3 = self.v3 + v_1.v3;
    }
    //mul num to vec
    pub fn Mul_Self_Vec(&mut self,val: f64){
	self.v1 = self.v1*val;
	self.v2 = self.v2*val;
	self.v3 = self.v3*val;
    }
    //return a mul self vec
    pub fn Mul_Val_Vec(&mut self,v : Vec3 , val : f64){
	self.v1 = v.v1*val;
	self.v2 = v.v1*val;
	self.v3 = v.v1*val;
	
    }
    //calc unit vector
    pub fn Unit_Self_Vec(&mut self,t_v1 : Vec3){
	
	let length : f64 = (t_v1.v1*t_v1.v1 +
			    t_v1.v2*t_v1.v2 + t_v1.v3*t_v1.v3).sqrt();

	if length > 0.0{
	    self.v1 = t_v1.v1 / length;
	    self.v2 = t_v1.v2 / length;
	    self.v3 = t_v1.v3 / length;
	}
    }
    pub fn Get_V1(&self)->f64{
	let t : f64 = self.v1;
	return t;
    }
    
    pub fn Get_V2(&self)->f64{
	let t : f64 = self.v2;
	return t;
    }
    //mul a num to a vec
    pub fn Mul_Num_Vec(&mut self,val: f64, v1 : Vec3){
	self.v1 = val*v1.v1;
	self.v2 = val*v1.v2;
	self.v3 = val*v1.v3;
    }
    //now convert the vec3 values into color[0,255]
    pub fn Convert_To_Color(&mut self){

	self.v1 = self.v1.clamp(0.0, 1.0);
	self.v2 = self.v2.clamp(0.0, 1.0);
	self.v3 = self.v3.clamp(0.0, 1.0);
	
	// Convert to [0, 255]
	self.vec_r = (255.999 * self.v1) as i64;
	self.vec_g = (255.999 * self.v2) as i64;
	self.vec_b = (255.999 * self.v3) as i64;
    }
    //print color of vec
    pub fn Print_Color(&self){
	println!("r: {} g: {} b: {}",self.vec_r,
		 self.vec_g,self.vec_b);
    }
    //return rgb
    pub fn Get_R(&self)->i64{
	return self.vec_r;
    }
    pub fn Get_G(&self)->i64{
	return self.vec_g;
    }
    pub fn Get_B(&self)->i64{
	return self.vec_b;
    }
    //print colors
    pub fn Print_Color_Vec(&self){
	println!("Color R: {} G: {} B: {}",self.vec_r,
		 self.vec_g,self.vec_b
	)
    }
    
    //dot product of two vec
    pub fn Dot_Vec_Vec(&self,v1 : Vec3,v2 : Vec3)->f64{
	let temp : f64 = (v1.v1 * v2.v1
			  + v1.v2 * v2.v2
			  + v1.v3 * v2.v3
	);

	return temp;
    }
    //check if hit sphere func
    pub fn Hit_Sphere(&self,ray_direction : Vec3) -> bool{
	//sphere centre
	let ray_origin : Vec3 = Self::Init_Vec(0.0,0.0,0.0);
	let sphere_centre : Vec3 = Self::Init_Vec(0.0,0.0,-1.0);
	let radius : f64 = 0.5;

	let mut unit_direction : Vec3 = Self::Init_Vec(0.0,0.0,0.0);
	unit_direction.Unit_Self_Vec(ray_direction.clone());
	//start calc
	let mut CP : Vec3 = Self::Init_Vec(0.0,0.0,0.0);
	CP.Sub_Vec_Vec(ray_origin.clone(),sphere_centre.clone());
	
	let a : f64 = self.Dot_Vec_Vec(unit_direction.clone(),
					unit_direction.clone()
	);
	let b : f64 = -2.0 * self.Dot_Vec_Vec(unit_direction.clone(),
					      CP.clone()
	);
	let c : f64 = self.Dot_Vec_Vec(CP.clone(),CP.clone()) - (radius*radius);

	let discriminant : f64 = b*b - 4.0*a*c;
	println!("\na val : {}",a);
	println!("b val : {}",b);
	println!("c val : {}",c);
	println!("discriminant  val : {}",discriminant);
	if discriminant >= 0.0 {
	   
	    return true;
	}else{
	    return false;
	}
	return false;
    }
    //calculate color and return
    pub fn Calculate_Color(&self,ray_direction : Vec3,
			   blend_col1 : Vec3,blend_col2 : Vec3,
			   r : i32
    )->Vec3{

	//check if hits the sphere if yes then return red
	let is_hit : bool  = self.Hit_Sphere(ray_direction.clone());
	if is_hit {
	    let temp_color_3 : Vec3 = Self::Init_Vec(1.0,0.0,0.0);
	    return temp_color_3;
	}
	let mut unit_direction : Vec3 = Self::Init_Vec(0.0,0.0,0.0);
	let mut temp_color_1 : Vec3 = Self::Init_Vec(0.0,0.0,0.0);
	let mut temp_color_2 : Vec3 = Self::Init_Vec(0.0,0.0,0.0);
	let mut temp_color_3 : Vec3 = Self::Init_Vec(0.0,0.0,0.0);
	
	//currently directly using the ray_direction
	unit_direction.Unit_Self_Vec(ray_direction.clone());
	
	//now applying the linear blend
	//let a : f64 = 0.5 * (unit_direction.Get_V2() + 1.0);
	let a : f64 = r as f64 / 144.0; 
	
	//println!("\na value: {}",a);
	
	//---now the main color
	temp_color_1.Mul_Num_Vec(1.0-a,blend_col1.clone());
	temp_color_2.Mul_Num_Vec(a,blend_col2.clone());
	
	temp_color_3.Sum_Vec_Vec(temp_color_1.clone(),temp_color_2.clone());
	
	temp_color_3.v1 = temp_color_3.v1.clamp(0.0, 1.0);
	temp_color_3.v2 = temp_color_3.v2.clamp(0.0, 1.0);
	temp_color_3.v3 = temp_color_3.v3.clamp(0.0, 1.0);
	
	return temp_color_3;
	
    }
}
