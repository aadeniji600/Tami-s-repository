  // Rust program to calculate speed of a car in kilometers
  
  use std::io;

  fn main() {

     let mut input1 = String::new();
     let mut input2 = String::new();

     println!("Enter distance in miles");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let d:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter time in hours");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let t:f32 = input2.trim().parse().expect("Not a valid number");

     let mut speed:f32 = d / t;
     speed = speed.sqrt();


     println!("Speed of a car: {}", speed);
    
}
