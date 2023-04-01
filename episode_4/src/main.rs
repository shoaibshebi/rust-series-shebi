#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let mut user1 = User{
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    // println!("User1: {:?}", user1);

    // build user
    let user2 = build_user(String::from("user2"), String::from("user2@gmail.com"));

    println!("User2: {:?}", user2);


    // update user
    user1.username = String::from("user1_updated");

    // tuple struct
    struct Color(i32, i32, i32);
    struct Rectangle(i32, i32);

    let black = Color(0, 0, 0);

    // moving values

    let user3 = User{
        username: String::from("user3"),
        email:  String::from("user@gmail.com"),
        sign_in_count: user1.sign_in_count,
        active: true,
    };

    println!("User3: {:?}", user3);
    println!("User1: {:?}", user1.sign_in_count);


    example_program();
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle{
    height: i32,
    width: i32,
}
impl Rectangle {
    fn new(height: i32, width: i32) -> Rectangle{
        Rectangle{
            height,
            width,
        }
    }
    fn area(&self) -> i32{
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.height > other.height && self.width > other.width
    }
    fn width(&self) -> i32{
        self.width
    }
}

impl Rectangle{
    fn height(&self) -> i32{
        self.width
    }
}

fn example_program(){
    let height1 = 50;
    let width1 = 30;

    println!("The area of the rectangle is {} square pixels.", area(height1, width1));

    let rect1 = (50, 30);

    println!("The area of the rectangle is {} square pixels.", area_tuple(rect1));

    let rect2 = Rectangle{
        height: 50,
        width: 30,
    };

    // println!("The area of the rectangle is {} square pixels.", area_struct(rect2));

    let area_calculated = rect2.area();
    print!("The area of the rectangle via method on Struct is {} square pixels.", area_calculated);

    let rect3 = Rectangle{
        height: 40,
        width: 10,
    };

    let rect4 = Rectangle{
        height: 45,
        width: 60,
    };

    //compare these two rectangles
    let can_hold = rect3.can_hold(&rect4);
    println!("Can rect3 hold rect4? {}", can_hold);
    
    let with = rect3.width();
    let with2 = rect4.width;

    // accossiate function
    let rect5 = Rectangle::new(50, 30);

    println!("Rect5: {:?}", rect5);

}
fn area(height: i32, width: i32) -> i32{
    height * width
}
fn area_tuple(dimensions: (i32, i32)) -> i32{
    dimensions.0 * dimensions.1
}
fn area_struct(rectangle: Rectangle) -> i32{
    rectangle.height * rectangle.width
}