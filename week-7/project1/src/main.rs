  // Rust program to perfom certain calculations

  use std::io; 

  fn trapezium() {
    //to find area of a trapezium
    println!("Area of a trapezium");

     let mut height = String::new();
     let mut base1 = String::new();
     let mut base2 = String::new();

     println!("Enter height");
     io::stdin().read_line(&mut height).expect("Not a valid string");
     let input1:f32 = height.trim().parse().expect("Not a valid number");

     println!("Enter base1 of the trapezium");
     io::stdin().read_line(&mut base1).expect("Not a valid string");
     let input2:f32 = base1.trim().parse().expect("Not a valid number");

     println!("Enter base2 of the trapezium");
     io::stdin().read_line(&mut base2).expect("Not a valid string");
     let input3:f32 = base2.trim().parse().expect("Not a valid number");

     let area = (input1 / 2.0) * (input2 + input3);

      println!("Area of a trapezium: {} cm^2.", area);
    
}

 fn rhombus() {
    //to find the area of a rhombus
    println!("Area of a rhombus");
     
     let mut diagonal1 = String::new();
     let mut diagonal2 = String::new();

     println!("Enter diagonal1 of the rhombus");
     io::stdin().read_line(&mut diagonal1).expect("Not a valid string");
     let input1:f32 = diagonal1.trim().parse().expect("Not a valid number");

     println!("Enter diagonal2 of the rhombus");
     io::stdin().read_line(&mut diagonal2).expect("Not a valid string");
     let input2:f32 = diagonal2.trim().parse().expect("Not a valid number");

     let area = 0.5 * input1 * input2;

      println!("Area of a rhombus: {} cm^2", area);
 }

 fn parallelogram() {
    //to find the area of a parallelogram
    println!("Area of a parallelogram");
     
     let mut base = String::new();
     let mut altitude = String::new();

     println!("Enter base of the parallelogram");
     io::stdin().read_line(&mut base).expect("Not a valid string");
     let input1:f32 = base.trim().parse().expect("Not a valid number");

     println!("Enter altitude of the parallelogram");
     io::stdin().read_line(&mut altitude).expect("Not a valid string");
     let input2:f32 = altitude.trim().parse().expect("Not a valid number");

     let area = input1 * input2;

      println!("Area of a parallelogram: {} cm^2.", area);
 }

 fn cube() {
     let mut length = String::new();

     println!("Enter length of the side of the cube");
     io::stdin().read_line(&mut length).expect("Not a valid string");
     let input1:f32 = length.trim().parse().expect("Not a valid number");


     let area = 6.0 * input1 * input1;

      println!("Area of a cube: {} cm^2.", area); 
 }

 fn cylinder() {
    //to find volume of a cylinder
    println!("Volume of a cylinder"); 
    
     let mut radius = String::new();
     let mut height = String::new();

     println!("Enter the radius of the cylinder");
     io::stdin().read_line(&mut radius).expect("Not a valid string");
     let input1:f32 = radius.trim().parse().expect("Not a valid number");

     println!("Enter the height of the cylinder");
     io::stdin().read_line(&mut height).expect("Not a valid string");
     let input2:f32 = height.trim().parse().expect("Not a valid number");

     let volume = (22.0 / 7.0) * input1 * input1 * input2;

      println!("Volume of a cylinder is: {} cm^2.", volume);

}

fn main() {
   //Program to calculate the area and volume of certain shapes
   println!("Welcome");

   println!("Press 1 if you want to find the area of a trapezium ");
   println!("Press 2 if you want to find the area of a rhombus ");
   println!("Press 3 if you want to find the area of parallelogram ");
   println!("Press 4 if you want to find the area of a cube ");
   println!("Press 5 if you want to find the volume of a cylinder ");
   let mut i91 = String::new();
   io::stdin().read_line(&mut i91).expect("Not a valid string");
   let _i91:i32 = i91.trim().parse().expect("Not a valid number");

  if _i91 == 1 { 
     trapezium();
   }

   if _i91 == 2 {
       rhombus();
   }

   if _i91 == 3 {
       parallelogram();
   }

   if _i91 == 4 {
       cube();
   }

   if _i91 == 5 {
       cylinder();
   }
   }

      



      
  