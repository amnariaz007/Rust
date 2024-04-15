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



fn main() {

    let x : u32 = 5;
    println!("x = {}", x);

    let x : &str = "Amna";
    println!("A = {}", x);

    let x : char = 'a';
    println!("A = {}", x); 

    let FLOAT_VALUE = 10.5;
    println!("FLOAT_VALUE = {}", FLOAT_VALUE); 

    let arr = [1,2,3,4,5];
    println!("{}", arr[0]); 

    let tup : (u32, bool, char) = (1, true, 'a');
    println!("tup = {}", tup.0); 


}

