 //Rust program to display a menu

  use std::io;

  fn main() {

    println!("Welcome to Cedar's Diner");

    let mut input1 = String::new();
    println!("Press 1 to see the menu");
    io::stdin().read_line(&mut input1).expect("Not a valid string");   
    let _menu:i32 = input1.trim().parse().expect("Not a valid number");

    let mut _input2 = String::new();
    println!("1. Poundo yam and edinkaiko
            \n2. Fried rice and chicken
            \n3. Amala and ewedu soup    
            \n4. Eba and egusi soup 
            \n5. White rice and stew  ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Choose a number from the menu");
    io::stdin().read_line(&mut _input2).expect("Not a valid string");
    let number:i32 = _input2.trim().parse().expect("Not a valid number");

    if number == 1
    {
        println!("1 portion of poundo yam and edinkaiko is N3200");
    }



}
