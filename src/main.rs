use std::{any::type_name_of_val};


fn main() {
    _method();
}
#[allow(unused_variables)]
fn _method(){
    struct Rectangle {
        width: i32,
        height: i32,
    }
    //Method
    impl Rectangle  {
        fn area(&self) -> i32 {
            return self.width * self.height
        }
        //Associated Function
        fn new() -> Self {
            Self { width: 10, height: 10 }
        }
    }

    let react1: Rectangle = Rectangle { width: 15, height: 20 };
    println!("{}", react1.area());

    let react2: Rectangle = Rectangle::new();
    println!("{}", react2.area());
}
fn _matching_id(){
    enum Messege {
        Hello {id: i32}
    }

    let msg: Messege = Messege::Hello {id: 19};

    match msg {
        Messege::Hello { id: _value @ (5 | 15) } => println!("Value: {}", _value),
        Messege::Hello { id: _etc @ (16..=20) } => print!("Between 6 - 20, value: {}", _etc),
        _ => println!("Nothing"),
    }
}

//Done
fn _shadowing(){
    let _age: Option<i32> = Some(18);
    //destructering
    if let Some(_age) = _age {
        println!("Age: {}", _age);
    } //or with match

    match _age {
        Some(age) => { println!("Age: {}", age)}
        _ => ()
    }
}   

//Done
fn _enum() {
    enum Direction {
        East,
        West,
        North,
        South
    }
    enum Number {
        One(u8),
        Two(u16),
    }
    //Multiple Data Type
    enum Messege {
        Quit,
        Move { x:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    //Enum
    let dire: Direction = Direction::East;
    //Option<T>
    let _number = Some(5u32);
    //Array Enum
    let _msgs: [Messege; 3] = [
        Messege::Quit,
        Messege::Move { x: (10), y: (5) },
        Messege::Write(String::from("Hello World"))
    ];
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => println!("North or South"),
        _ => ()
    };
    //Desturcturing some
    if let Some(angka) = _number {
        println!("{}", angka)
    }
    //enumerating enum
    for msg in _msgs {
        match msg {
            Messege::Quit => {
                println!("Quit")
            },
            Messege::Move { x: a, y: b } => {
                println!("{} {}", a, b)
            },
            Messege::Write(str) => {
                println!("{str}")
            },
            _ => (),
        }
    }
}

//Done Mirip Object: _struct();
struct  _User {
    active: bool,
    username: String,
    email: String,
}
fn _struct() {
    let user1 = _User {
        active: true,
        username: String::from("GG"),
        email: String::from("user@gmail.com"),
    };
    println!("{}, {}, {}", user1.active, user1.username, user1.email);
    let _user2 = _User {
        email: String::from("new@gmail.com"),
        ..user1
    };

    // println!("{}, {}, {}", user1.active, user1.username, user1.email); - Error karena moved value
}
//Done
fn _tuple() {
    let tup = (2,3);
    let (_deref1, _deref2) = tup;
    println!("{}, {}",  _deref1, _deref2)
}
//Done
fn _references() {
    let mut msg: String = String::from("Hello");

    let r1 : &mut String = &mut msg;
    println!("{},", r1);
    let r2: &mut String  = &mut msg;

    println!("{},", r2);
    // println!("{}, {}", r1, r2) -> Error
}
//done
fn _deref() {
    struct Person {
        _name: String,
        _age: Box<u8>,
    }

    let person: Person = Person {
        _name: String::from("Alice"),
        _age: Box::new(16),
    };
    //Deref
    let Person {_name, ref _age} = person;
    println!("{} {:?}", _name, person._age);
    // println!("Panic {:?}", person._name);

}
//Done 
fn _main2() {
    let s = String::from("Hello"); 
    let x: i8 = 10;
    _takes_ownership(s, x);
    println!("{:?}", x);
    // println!("Dropped{}", s);
}
fn _takes_ownership(allocated_string: String, makes_copy: i8) {
    println!("Alocated: {} | MakesCopy: {:?}", allocated_string, makes_copy);
}


//Done || println!("{}",_get_option(0))
fn _get_option(tp: u8) -> i32 {
    match tp {
        1 => {
            5i32
        }
        _ =>{
            10i32
        }
    }
}
//Done
fn _expression() {
    let x: u32 = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared
    };

    print!("{}", y)
}
//Done
fn _chartest() {
    let a: char = 'a';
    println!("{}", size_of_val(&a));
}
//Done
fn _floatingpoint() {
    let x: f64 = 0.2;
    let _y: f32 = 0.3;
    let z: f64 = 0.1;

    assert_eq!(type_name_of_val(&x), "f64".to_string());
    assert!(x as f32 + z as f32 == 0.3 as f32);
    
    for l in 'a'..='z' {
        println!("{}", l as u8);
    }
    println!("\nSuccess")
}
//Done
fn _palindrome()
{
    let _usernames:[&str; 4] = ["Tricer66", "TREXerT", "Veloc1COLEv", "stego95"];
    let _usernames:Vec<String> = _usernames.iter().map(|s| s.to_string()).collect();
    _findallpalindrome(_usernames);
    #[allow(unused_variables)]
    fn _findallpalindrome(_usernames: Vec<String>){
        let mut result:Vec<&String> = Vec::new();
        for name in &_usernames {
            let _vec: Vec<char> = name.to_lowercase().chars().collect();
            let _findletter = || {
                for i in 0.._vec.len(){
                    if _vec[i] == _vec[_vec.len() -1 -i] {
                        continue;
                    } else {
                        return false;
                    }
                }
                true
            };
            if _findletter() == false {
                continue;
            } else {
                result.push(name);
            }
        }
        println!("{:?}", result)

    }
}

//Done
fn _string_task(){
    let _string_1 = String::from("Hello");
    let _array_string_2: [String; 3] = [String::from("One"),String::from("two"),String::from("three")]; 
    println!("{}", _string_1);
    //reverse
    let _array: Vec<char> = _string_1.chars().collect(); //Memisahlan
    let _array_2: Vec<&str> = _string_1.split_whitespace().collect(); //Per Whitespace
    println!("{:?}\n{:?}", _array, _array_2);
    let mut _reversed = String::new();
    for i in (0.._array.len()).rev(){
        _reversed.push(_array[i]);
    }
    println!("{}", _reversed);
}
//Done
fn _range_check(arr: &[i32]) -> i32 {
    return *arr.iter().max().unwrap() - *arr.iter().min().unwrap();
}
//Done
fn _exponential_array(){
    let arr_1 = [1, 2, 4, 8, 16];
    let arr_2: [i32; 5] = [1,1,1,1,1];
    for i in arr_1 {
        for j in arr_2 {
            print!("{} \t", i * j);
            
        }
        print!("\n");
    }
}
//Done
fn _task_one() {
    let mut x: i32 = 3;
    x += 2;
    assert_eq!(x, 5);
    println!("Success");
}
//Done
#[allow(unused_variables)]
fn _define_x() {
    let x: &str = "Hello";
    println!("{}, World", x);
}