// fn main() {
//     println!("Hello, world!");
//     let mut x = 5;
//     println!("the value is {x}");

//     x = 6;
//     println!("the value is {x}");
// }

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value1 is {x}");
    }

    println!("the value2 is {x}");
}
