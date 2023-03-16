fn main() {
        // let x = student_age(18)
        // println!("The student age is {}", x);

        control_flow();
}


// signature of the function: i-function name, ii-no of paramerts iii- type of parameters
fn student_age(age:u32) -> u32  {
    // println!("The student age is {}", age);

    // let x = 30;
    // let  y = 10;

    // let y = {
    //     let x = 30;
    //     x + 10
    // }

    age
} 

fn control_flow(){
    // if else
    let x = 10;
    if x > 10 {
        println!("x is greater than 10");
    } else if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is equal to 10");
    }

    if x != 0  {
        println!("x is true");
    }        

    // if in let statment
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };   

    println!("The value of number is: {}", number);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    //while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    //for loop
    let a = [10, 20, 30, 40, 50];
    for item in a.rev() {
        println!("the value is: {}", item);
    }
    
}