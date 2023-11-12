  //Rust program to display a menu

  use std::io

  fn main() {

    println!("Welcome to Cedar's Diner.\nThis is our menu");
    println!("\nA portion of poundo yam/edinkaiko soup is N3200");
    println!("\nA portion of fried rice and chicken is N3000");
    println!("\nA portion of amala and ewedu soup is N2500");
    println!("\nA portion of eba and egusi soup is N2000");
    println!("\nA portion of white rice and stew is N2500");

    println!("");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("\nEnter the portion of poundo yam and edinkaiko you would like");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("\nEnter the portion of fried rice and chicken you would like");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("\nEnter the portion of amala and ewedu soup you would like");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    println!("\nEnter the portion of eba and egusi soup you would like");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let d:f64 = input4.trim().parse().expect("Not a valid number");

    println!("\nEnter the portion of white rice and stew you would like");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let e:f64 = input5.trim().parse().expect("Not a valid number");

    let pcost:f64 = 3200.0;
    let fcost:f64 = 3000.0;
    let acost:f64 = 2500.0;
    let ecost:f64 = 2000.0;
    let wcost:f64 = 2500.0;

    let p = input1 * acost;

    let f = input2 * bcost;

    let a = input3 * ccost;

    let e = input4 * dcost;

    let w = input5 * ecost;

    let total = p + f + a + e + w;

    if total > 10000.0 {
        let discount = 0.95 * bill;
        println!("\nYou have a discount of 5% and your new bill is {}",discount );
    }

    else {
        println!("\nYour total bill is {}",bill );
    }
}
