use std::any::type_name_of_val;

fn main() {
    _floatingpoint();
}
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