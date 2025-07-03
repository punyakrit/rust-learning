//  fn main() {

//     // let number :i32 = -1000000;

//     // println!("hello world"); // this is a macro
//     // let ans = sum(1, 2);
//     // println!("{}", ans);


//     // let ans  = is_even(20);
//     // println!("{}", ans);

//     let name = String::from("Annop");
//     println!("First Name - {}", name);


//     let name_vector = vec![1,2,30];

//     println!("{:?}" , name_vector);

// }

// // fn sum(a: u8, b: u8) -> u8 {
// //     return a + b;
// // }

// // fn is_even(a:u32) -> bool {
// //     return a%2 == 0;
// // }

// fn main(){
//     let is_number = 91;

//     let is_even = is_evens(is_number);

//     if is_even {
//         println!("{} it is even number" , is_number);
//     }else{
//         println!("it is odd number");
//     }

//     print_loop();
// }

// fn print_loop(){
//     for i in 1..10 {
//         println!("{} " , i);
//     }
// }


// fn is_evens(number:u32) -> bool{
//     return number%2 == 0;
// }


// fn main(){

//     // everything in rust is immutable to make it mutbale add "mut" keyword
//     let mut x = 10;
//     x=11;

//     println!("{}" , x);

//     let mut name = String::from("Hello");

//     name.push_str(" world");

//     println!("{}" , name)
// }



// fn main(){
//     create_str();

// }

// fn create_str(){
//     let name = String::from("Anoop");

//     let name2 = name; //if here name2 = name the original name poiting to its value in heap is deallocated and its new owner is name

//     println!("{}", name); //ownership rule
// }


// fn main(){
//     let name = String::from("Anoop");
//     // let name1 = name.clone();
//     let (length, name) = str_length(name);
//     println!("{}" , length);

//     println!("{}",name); //in here this would throw error as in function call we pass in name to s variable as owner and as per its rule when function call ended its value got dropped
// }

// fn str_length(s:String) ->(usize, String){
//     return (s.len(), s);
// }



//References

// fn main(){
//     let text = String::from("Random");
//     let text2 = &text; //text2 saves address of string instead text transfering ownership of string
//     let text3 = &text;
//     println!("{}", text);
//     println!("{}", text2);
// }


//borrow

// fn main(){
//     let s = String::from("no");
//     borrow_call(&s);
//     println!("Borrowed call {}" , s);
// }

// fn borrow_call(s: &String){
//     // s.push_str("sddsd");
//     println!("{}" , s);
// }


// Mutable references

// fn main(){
//     let mut s = String::from("Hello ");
//     call_mut(&mut s);
//     println!("{}" ,s);
// }

// fn call_mut(s: &mut String){
//     s.push_str(" world");
//     println!("{}", s);
// }

// -> one owner to string
// -> one owner -> -> multiple references
// -> one mutable owner can have only one mutable references not more than that


// Struct in Rust

// use rect::Rect;
// pub mod rect;

// fn main(){
//     let r = Rect{
//         weight : 10.0,
//         height:10.0
//     };
//     println!("{} {}" , r.height , r.weight);
//     println!("{}" ,r.area() );
//     Rect::print_something();


// }



//Enums
// enum  Direction {
//     North,
//     South,
//     East,
//     West
// }

// fn main(){
//     let direction= Direction::East;
//     steer(direction);
// }

// fn steer(dir: Direction){
//     match dir {
//         Direction::North => println!("North"),
//         Direction::South => println!("South"),
//         // Direction::East => println!("East"),
//         Direction::West => println!("West"),
//         _=>println!("NONE"),
//     }
// }