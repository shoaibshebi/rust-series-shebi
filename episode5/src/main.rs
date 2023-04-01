fn main(){
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            
        }
        
    }

    let msg = Message::Write(String::from("hello"));

    // Option

    let some_number :Option<i32> = Some(5);
    let none :Option<i32> = None;

    let x = 6;

    // let sum = some_number+ x;

    // coin example

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let resutl = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("result: {}", resutl);




fn value_in_cents(coin: Coin) -> u8 {
    // coin:: Penny

    // match coin {
    //     Coin::Penny => {
    //         println!("Lucky Penny!");
    //         1
    //     },
    //     Coin::Nickel => 5,
    //     Coin::Dime => 10,
    //     Coin::Quarter(state) => {
    //         println!("State quarter from {:?}!", state);
    //         25
    //     }
    // }

    // option

    let five = Some(5);
    
    match five {
        Some(5) => 5,
        _ => 0,
    }

}

    let some_val =5;

    match some_val {
        5 => println!("five"),
        6 => println!("six"),
        7 => println!("7"),
        _ => println!("other"),
    }

    // if let

    let some_value = Some(5);

    if let Some(x) = some_value {
        println!("x {}", x);
    } else {
        println!("not three");
    }

}
