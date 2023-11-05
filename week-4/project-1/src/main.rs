  // Rust program to calculate speed of a car in kilometers
  
  use std::io;

  fn main() {

     let mut input1 = String::new();
     let mut input2 = String::new();

     println!("Enter distance in miles");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let a:f32 = input1.trim().parse().expect("Not a valid number");
    
}
