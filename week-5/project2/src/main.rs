  //Rust program to determine the annual incentive of employees
  //using the age and the experience of the person

  use std::io;

  fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter number of years of experience: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string number");
    let experience:i32 = input2.trim().parse().expect("Not a valid number");

    if age >= 40 && experience >= 5 
    {
       println!("\nYour incentive is 1_560_0000");
    }
    else if age >= 30 && age < 40 && experience >= 5
    {
        println!("\nYour incentive is 1_480_000");
    }
    else if age < 28 && experience >= 5 
    {
        println!("\nYour incentive is 1_300_000");
    }
    else if experience < 5 
    {
        println!("\nYour incentive is 100_000");
    }


  }
