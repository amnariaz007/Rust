use std::io;

// fn main() {
//     println!("Hello, world!");
// }


// fn main(){
//     let mut x = 5;
//     println!("x = {}", x);

//     x = x+1;
//     println!("x = {}", x);

//     {
//         let x = x+1;
//     println!("x = {}", x);
//     }
// }



// fn main() {

//     let x : u32 = 5;
//     println!("x = {}", x);

//     let x : &str = "Amna";
//     println!("A = {}", x);

//     //char
//     let x : char = 'a';
//     println!("A = {}", x); 

//     //Floating
//     let FLOAT_VALUE = 10.5;
//     println!("FLOAT_VALUE = {}", FLOAT_VALUE); 

//     //Array
//     let arr = [1,2,3,4,5];
//     println!("{}", arr[0]); 
//     //Tuples
//     let tup : (u32, bool, char) = (1, true, 'a');
//     println!("tup = {}", tup.0); 


//     // console user input output
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("failed to read");
//     println!("{}", input);
    
//     sigle_parammeter(69);
//     multiple_para(5,5);

// }


// fn sigle_parammeter( x: u32){
//     println!("the value of x is {x}")
// }


// fn multiple_para(x: u32, y: u32){
//   let z = x*y;
//     println!(" The value of z is {z}")
// }

fn main () {
   let x = return_function();
   println!("X is this {}", x);
}


fn return_function () -> u32 {
    50 * 50 
}