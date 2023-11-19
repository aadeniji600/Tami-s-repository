//Rust program to ask for patient information at a hospital

use std::io;

fn main() {
    println!("Welcome to Otunene Family Health Center,
              \nAnswer the following questions");

  println!("Enter your name : ");
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).expect("Not a valid string");

  println!("Enter the date of the day you were born eg 14, 23 : ");
  let mut i10 = String::new();
  io::stdin().read_line(&mut i10).expect("Not a valid string");

  println!("Enter the month of the year you were born .  ");
  let mut i20 = String::new();
  io::stdin().read_line(&mut i20).expect("Not a valid string");

  println!("Enter the year you were born : ");
  let mut i30 = String::new();
  io::stdin().read_line(&mut i30).expect("Not a valid string");
  let _year:i64 = i30.trim().parse().expect("Not a valid number");

  println!("Enter email address : ");
  let mut input2 = String::new();
  io::stdin().read_line(&mut input2).expect("Not a valid string");

  println!("Enter phone number : ");
  let mut i40 = String::new();
  io::stdin().read_line(&mut i40).expect("Not a valid string");
  let _phonenumber:f64 = i40.trim().parse().expect("Not a valid number");

  println!("Enter number of siblings (Please put zero if you have none)");
  let mut i50 = String::new();
  io::stdin().read_line(&mut i50).expect("Not a valid string");
  let _siblings:i32 = i50.trim().parse().expect("Not a valid number");

  println!("Enter number of children (Please put zero if you have none)");
  let mut i60 = String::new();
  io::stdin().read_line(&mut i60).expect("Not a valid string");
  let _children:i32 = i60.trim().parse().expect("Not a valid number");

  println!("Enter your age");
  let mut i70 = String::new();
  io::stdin().read_line(&mut i70).expect("Not a valid string");
  let _age:f64 = i70.trim().parse().expect("Not a valid number");

  println!("Press 1 if you are part of the first 100 people to receive care today");
  println!("Press 2 if you're not part of the first 100 people to receive care today");
  let mut i80 = String::new();
  io::stdin().read_line(&mut i80).expect("Not a valid string");
  let _i80:i32 = i80.trim().parse().expect("Not a valid number");

  println!("Press 1 if your medical diagnosis is alzheimer");
  println!("Press 2 if your medical diagnosis is arrythmia");
  println!("Press 3 if your medical diagnosis is chronic kidney disease");
  println!("Press 4 if your medical diagnosis is diabetes");
  println!("Press 5 if your medical diagnosis is arthritis");
  let mut i90 = String::new();
  io::stdin().read_line(&mut i90).expect("Not a valid string");
  let _i90 :i32 = i90.trim().parse().expect("Not a valid number");

  println!("Press 1 if you stay in Akpabom ");
  println!("Press 2 if you stay in Ngbauji ");
  println!("Press 3 if you stay in Atabrikang ");
  println!("Press 4 if you stay in Okorobilom ");
  println!("Press 5 if you stay in Emeremen ");
  println!("Press 6 if you live elsewhere");
  let mut i91 = String::new();
  io::stdin().read_line(&mut i91).expect("Not a valid string");
  let _i91:i32 = i91.trim().parse().expect("Not a valid number");

  let a:f64 = 1_200_000.00;
  let b:f64 = 550_000.00;
  let c:f64 = 1_500_000.00;
  let d:f64 = 800_000.00;
  let e:f64 = 450_000.00; 

  //display info
  println!("Your name is {} ", input1);
    println!("Date of birth: {}/{}/{} ", i10,i20,i30);
  println!("Email address : {}",input2 );
  println!("Phone number : {}",i40 );
  println!("You have {} siblings and {} children",i50, i60 );

  if _i90 == 1 && _age > 50.0 && _children > 4 && _i91 == 1{
    let bill = a * 0.8;

    println!("You have received a 20% discount"); 
    println!("Your bill is :{}", bill);
  }
  else {
        println!("Your bill is {}", a);
      
  }

  if _i90 == 2 && _age == 30.0 && _siblings > 4 && _i91 == 2{
    let bill = b * 0.95;

    println!("You have received a 5% discount"); 
    println!("Your bill is :{} ", bill);
  }
  else {
    println!("Your bill is :{}", b);
  }

  if _i90 == 3 && _age > 40.0 && _children > 3 && _siblings > 3 && _i91 == 3{
    let bill = c * 0.85;

    println!("You have received a 15% discount"); 
    print!("Your bill is {}", bill);
  }
  else {
    println!("Your bill is :{}", c);
  }

  if _i90 == 4 && _age > 28.0 && _age < 45.0 && _children >= 2 && _children <= 4 
  && _siblings == 0 && _i91 == 4{
    let bill = d * 0.90;

    println!("You have received a 10% discount"); 
    println!("Your bill is :{}", bill);
   }
   else {
     println!("Your bill is :{}", d);
   }

   if _i90 == 5 && _age > 58.0 && _children > 5 &&  _siblings > 5 && _i91 == 5{
    let bill = e * 0.0;

    println!("You have received a 10% discount"); 
    println!("Your bill is :{}", bill);
   }
   else {
     println!("Your bill is :{}", e);
   }












}
