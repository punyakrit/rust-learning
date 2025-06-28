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

fn main(){
    let is_number = 91;

    let is_even = is_evens(is_number);

    if is_even {
        println!("{} it is even number" , is_number);
    }else{
        println!("it is odd number");
    }

    print_loop();
}

fn print_loop(){
    for i in 1..10 {
        println!("{} " , i);
    }
}


fn is_evens(number:u32) -> bool{
    return number%2 == 0;
}
