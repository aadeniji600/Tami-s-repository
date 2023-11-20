 use std::io;

 fn main() {


  println!("Welcome to the Student Council Voting System");

  println!("Enter your name : ");
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Not a valid string");

  println!("Enter your email address : ");
  let mut input2 = String::new();
  io::stdin().read_line(&mut input2).expect("Not a valid string");

  println!("Enter your department : ");
  let mut input3 = String::new();
  io::stdin().read_line(&mut input3).expect("Not a valid string");

  println!("Enter your state of origin : ");
  let mut input4 = String::new();
  io::stdin().read_line(&mut input4).expect("Not a valid string");

  println!("Click 1 if you are a current class representative");
  let mut i1 = String::new();
  io::stdin().read_line(&mut i1).expect("Not a valid string");
  let _i1 :i32 = i1.trim().parse().expect("Not a valid number");

  println!("Click 1 if you are in 100 level and 0 if not");
  let mut i2 = String::new();
  io::stdin().read_line(&mut i2).expect("Not a valid string");
  let _i2 :i32 = i2.trim().parse().expect("Not a valid number");

  println!("Click 1 if your CGPA is above 4.0");
  let mut i3 = String::new();
  io::stdin().read_line(&mut i3).expect("Not a valid string");
  let _i3 :i32 = i3.trim().parse().expect("Not a valid number");


  //display info
  println!("Your name is {} ", input1);
  println!("Your department is {} ", input3);
  println!("Your email adrress is {}",input2 );
  println!("Your state of origin is {}", input4);

  if _i1 == 1 && _i2 == 0 && _i3 == 1 {
      println!("You can vote");
  }
  else {
      println!("Sorry, you are not eligible to vote.");
  }







 



     
 }
