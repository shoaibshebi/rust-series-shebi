fn main() {
    let x = 5;
    let y = x;

    println!("x: {}, y: {}", x, y);

    // let mut shoaib = String::from("car");
    // let adam = &mut shoaib;
    // let john = &shoaib;

    // println!("shoaib: {}, adam: {}", shoaib, adam);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1: {}, s2: {}", s1, s2)

    // {
    //     let s = String::from("hello");
    //     println!("s: {}", s);
    // }

    // println!("s: {}", s)

    // let s = String::from("hello");
    // let somother = takes_ownership(s);

    // print!("s: {}", somother);

    // let mut s = String::from("hello");
    //     change(&mut s);

    // let mut s = String::from("hello");

    // let r1 = &s; // hello
    // let r2 = &s; // hello
    // println!("{} and {}", r1, r2); // hello , hello

    // let r3 = &mut s;  // hello

    // r3.push_str(" world");
    // // println!("{}", r3);
    // print!("r3: {}, ", r3)


    //  Dandling pointer
    // let s = String::from("hello");

    // let reference = dangle();

    // println!("reference: {}", reference);


    // Slice type

    let s = String::from("hello world");
    let word = first_word(&s);
    
    println!("word: {}", word);


    //string literal
    let name = "shoaib";
    // shoaib 
    // 012345
    let first_3_letters = &name[0..3];


    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
}

fn takes_ownership(some_string: String) -> String {
    println!("some_string: {}", some_string);

    some_string
}
fn change(some_string: &mut String) {
    some_string.push_str(" world");
}

fn dangle() -> String {
    let s = String::from("hello");

    s // returned
} // Droped

fn first_word(s: &String) -> &str {

    //hello world
    // hello

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {


        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}