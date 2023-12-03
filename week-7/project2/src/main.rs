//Rust programt to ask the number of siblings from a client

use std::io;

fn main() {
  println!("Welcome to The National Information Center
              \nAnswer the following questions");

  println!("Enter your name : ");
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Not a valid string");

  println!("Enter the number of siblings you have
            \n type zero if you have none");
  let mut i10 = String::new();
  io::stdin().read_line(&mut i10).expect("Not a valid string");

  println!("Enter the first name of your siblings use and
            \nto seprate the names eg Kiki and Damope: ");
  let mut input2 = String::new();
  io::stdin().read_line(&mut input2).expect("Not a valid string");

  println!("Enter the age of your siblings use and to
            \nseprate them if you have more than one eg 18 and 19.  ");
  let mut i20 = String::new();
  io::stdin().read_line(&mut i20).expect("Not a valid string");
  let _age:i64 = i20.trim().parse().expect("Not a valid number");


  println!("Are any of your siblings greater than 18?
            \nif yes press 1 for single and 2 for married press
            \n0 if they are not greater than 18. ");
  let mut i30 = String::new();
  io::stdin().read_line(&mut i30).expect("Not a valid string");
  let _age:i64 = i30.trim().parse().expect("Not a valid number");

  println!("If your previous answer was 1 is your siblings
           \na student or a worker press 1  for student and 2
           \n for worker. ");
  let mut i40 = String::new();
  io::stdin().read_line(&mut i40).expect("Not a valid string");
  let _occupation:i64 = i30.trim().parse().expect("Not a valid number");

  println!("If your previous answer was 1 what university
           \nand course of study is your sibling in. ");
  let mut input3 = String::new();
  io::stdin().read_line(&mut input3).expect("Not a valid string");

  
  

}
