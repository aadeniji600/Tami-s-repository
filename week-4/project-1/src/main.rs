  // Rust program to calculate speed of a car in kilometers
  
  use std::io;

  fn main() {

     let mut input1 = String::new();
     let mut input2 = String::new();

     println!("Enter distance in miles");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let d:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter time in hours");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
     let t:f32 = input2.trim().parse().expect("Not a valid number");

     let d_km = d * 1.609;
     let speed:f32 = d_km / t;

     println!("Speed of a car: {}", speed);
    
}
