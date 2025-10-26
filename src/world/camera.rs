pub struct Camera{
    pub center_x: f32,
    pub center_y: f32,
    pub width: u16,
    pub height: u16,
}

impl Camera {
    pub fn new(center_x: f32, center_y: f32, width: u16, height: u16) -> Self{
        Self { center_x: center_x, center_y: center_y, width: width, height: height}
    }  
    
    pub fn is_visible(x: f32, y: f32) -> bool {
        return ((abs(self.center_x-x)<width/2) && (abs(self.center_y-y)<height/2))
    }
}