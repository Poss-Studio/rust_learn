struct Rectangle{
    width:i32,
    height:i32,
}
impl Rectangle{
    fn area(self) -> i32{
        self.width * self.height
    }
    fn cube(size:i32) -> Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
    fn can_hold(&self,other:&Rectangle) -> bool{
        self.width> other.width && self.height > other.height
    }
}
pub fn sample(){
    let rexim_rc = Rectangle{
        width:50,
        height:70
    };
    println!("The value of the Area is {}",Rectangle::area(rexim_rc));
}
