#[derive(Clone,Debug)]
pub struct Vec3{
    v1 : f64,
    v2 : f64,
    v3 : f64,
}

impl Vec3{
    
    pub fn Init_Vec(v1: f64, v2: f64 ,v3: f64)->Self{
	Vec3{v1 : v1,v2: v2, v3: v3 }
    }

    pub fn print_val(&self){
	println!("Val: {} {} {}",self.v1,self.v2,self.v3);
    }
    pub fn Div_Vec_Double(&mut self,viewport_u_v : &Vec3, image_width: f64){
	self.v1 = viewport_u_v.v1 * (1.0/image_width);
	self.v2 = viewport_u_v.v2 * (1.0/image_width);
	self.v3 = viewport_u_v.v3 * (1.0/image_width);
	
    }
    //sub vec3 to vec3
    pub fn Sub_Vec_Vec(&mut self, p_vec : Vec3){
	self.v1 = self.v1  - p_vec.v1;
	self.v2 = self.v2  - p_vec.v2;
	self.v3 = self.v3  - p_vec.v3;
    }
    //sub vec3 to vec3 for upper left pixel
    pub fn Sub_Vec_Vec_ul(&mut self, p_vec : Vec3, val: f64){
	self.v1 = self.v1  - p_vec.v1/val;
	self.v2 = self.v2  - p_vec.v2/val;
	self.v3 = self.v3  - p_vec.v3/val;
    }
    //sum two vec3
    pub fn Sum_Vec_Vec(&mut self, v_1 : Vec3 , v_2 : Vec3){
	self.v1 = v_1.v1+v_2.v1;
	self.v2 = v_1.v2+v_2.v2;
	self.v3 = v_1.v3+v_2.v3;
    }
    //sum self to vec3
    pub fn Sum_Self_Vec(&mut self,v_1 : Vec3){
	self.v1 = self.v1 + v_1.v1;
	self.v2 = self.v2 + v_1.v2;
	self.v3 = self.v3 + v_1.v3;
    }
    //mul num to vec
    pub fn Mul_Num_Vec(&mut self,val: f64){
	self.v1 = self.v1*val;
	self.v2 = self.v2*val;
	self.v3 = self.v3*val;
    }
}
