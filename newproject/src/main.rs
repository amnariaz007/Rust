//use std::io;

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
//     println!("tup = {:?}", tup); 


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

//return functions 

// fn main () {
//    let x = return_function();
//    println!("X is this {}", x)
// }


// fn return_function () -> u32 {
//     50 * 50 
// }


//If else statements


// fn main () {
//     let no = 5;

//     if no == 5 {
//         println!("no is {}", no );
//     }else {
//         println!("no is not equal {}", no );
//     }
// }

// fn main (){

//     let name  = "Amna";

//     if name == "sana" {
//         println!("name is false {}", name );
//     }else if name == "naila"{
//         println!("name is false and {}", name );
//     }else {
//         println!("name is true and {}", name );
//     }
// }

// fn main ( ){

// let mut n = 10;

// while n < 1000 {
//     println!{ "n is : {}", n}

//     n +=20;
// }

// }

// fn main () {

//     let mut x = 2 ;
//     let mut y = 1;

//     while y <= 10{

//         let mul = x*y;
//         println!{ "mul is : {}", mul}

//         y +=1;
//     }


// }

// fn main () {
//     let x = 2;
//     let y = 1;

//     for y in 1..11 {
//         let mul = x*y;
//         println!("{} * {} = {}" , x, y, mul);
//     }
// }


fn main () {
    let x = 10;

    let y = x;
    println!(
        "y is : {}", y
    );
}




