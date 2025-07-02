pub struct Rect {
    pub height : f32,
    pub weight : f32
}

impl Rect {
    pub fn print_something(){
        println!("Printing Something");
    }

    pub fn area(&self) -> f32{
        return self.height * self.weight;
    }
}