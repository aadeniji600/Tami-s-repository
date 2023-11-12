  //Rust program to find the roots of a quadratic equation

  use std::io;

  fn main() {
    

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the values for 'a'  : ", );
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the values for 'b' : ", );
    io::stdin().read_line(&mut input2).expect("Not a valid string number");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter values for 'c' : ", );
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let d:f64 = b.powf(2.0) - 4.0*a*c;

    if d > 0.0
    {
        println!("\nThere are two distinct roots");
    }
    else if d == 0.0
    {
        println!("\nThere is exactly one real root");
    }
    else if d < 0.0 
    {
        println!("\nThere are no real roots");
    }

    let root1:f64 = (-b + d.powf(0.5)) / 2.0;
    let root2:f64 = (-b + d.powf(0.5)) / 2.0;

    println!("Roots of a quadratic equation are {} and {}", root1 ,root2 );
    }



