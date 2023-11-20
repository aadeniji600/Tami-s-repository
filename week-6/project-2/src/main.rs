use std::io;

 fn main() {


  println!("Welcome to the Researchers Publication Incentive Sysytem");

  println!("Enter your name : ");
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Not a valid string");

  println!("What number of papers have you published");
  let mut input2 = String::new();
  io::stdin().read_line(&mut input2).expect("Not a valid string");
  let _input2 :i32 = input2.trim().parse().expect("Not a valid number");

   //display info
  println!("Your name is {} ", input1);
  println!("The number of papaers you have published is {} ", input2);

  if _input2 >= 3 && _input2 <= 5  {
       println!("Your incentive is N500000");
   } 

  else if _input2 >= 5 && _input2 < 10{
    println!("Your incentive is N800000");
  }

  else if _input2 >= 10 {
    println!("Your incentive is N1000000");
  }

}
