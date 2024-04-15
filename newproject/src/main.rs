// fn main() {
//     println!("Hello, world!");
// }


fn main(){
    let mut x = 5;
    println!("x = {}", x);

    x = x+1;
    println!("x = {}", x);

    {
        let x = x+1;
    println!("x = {}", x);
    }
}

