use std::io;

fn main() {

    // Mutability 
    // let mut x:i32 = 10;

    // x = 9;

    // x = "shoaib";

    // Shadowing

    // let y = 10;

    // println!("y: {}", y);

    // {
    //     let y = y+1;
    //     // let x = 10;
    //     println!("y: {}", y);
    // }

    // println!("y: {}", y);


    // Data types
    //  i8, i16, i32, i64, i128, isize
    // u8, u16, u32, u64, u128, usize
    // f32, f64
    // bool
    // char

    // 1 - Scaller , 2- Compund


    // Wrap-around

    // let mut x:u8 = 255;
    // x = x + 1;

    // println!("x: {}", x);

    //https://doc.rust-lang.org/book/ch03-02-data-types.html#numeric-operations

    // let name:char = 'w';

    // let name:char = "saodfiha";
    // let name:char = String::new();

        // Compund

    // Tuple, Array

    // let tup = (1, 3.4 ,3);

    // let (x, y, z) = tup;

    // // println!("x: {}, y: {}, z: {}", x, y, z);
    // print!("{}", tup.0);

    // // Array

    let arr1 = [1,2,3];

    let arr2:[i32; 3] = [1,2,3];

    // Intresting array example

        let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("please input valid stuff");

    let guess:usize = guess.trim().parse().expect("not good input");

    let arrValue = arr1[guess];

}   
