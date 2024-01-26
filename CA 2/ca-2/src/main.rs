use std::io::Write;

  fn main() {

    let _company =           vec!["Company: Cadbury Nigeria Plc\n\n".to_string()];
    let _shares =            vec!["Shares: 15,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 5,500,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1965\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 63.33%\n\n".to_string()];


    let mut file = std::fs::File::create("CADBURY FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Cadbury Nigeria Plc\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }

    let _company =           vec!["Company: Champion Breweries PLC\n\n".to_string()];
    let _shares =            vec!["Shares: 25,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 8,500,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1975\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 68.00%\n\n".to_string()];


    let mut file = std::fs::File::create("CHAMPION FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Champion Breweries PLC\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }

    let _company =           vec!["Company: Dangote Sugar Refinery PLC\n\n".to_string()];
    let _shares =            vec!["Shares: 18,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 10,000,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1970\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 44.44%\n\n".to_string()];


    let mut file = std::fs::File::create("DANGOTE FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Sugar Refinery PLC\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }

    let _company =           vec!["Company: Flour Mills Nigeria PLC\n\n".to_string()];
    let _shares =            vec!["Shares: 32,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 4,500,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1960\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 87.50%\n\n".to_string()];


    let mut file = std::fs::File::create("FLOUR MILLS FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Flour Mills Nigeria PLC\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }

    let _company =           vec!["Company: Nestle Nigeria PLC\n\n".to_string()];
    let _shares =            vec!["Shares: 8,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 1,500,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1961\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 81.25%\n\n".to_string()];


    let mut file = std::fs::File::create("NESTLE FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Nestle Nigeria PLC\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }
    

    let _company =           vec!["Company: Unilever Nigeria PLC\n\n".to_string()];
    let _shares =            vec!["Shares: 37,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 11,000,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1923\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 70.27%\n\n".to_string()];


    let mut file = std::fs::File::create("UNILEVER FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Unilever Nigeria PLC\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }

    let _company =           vec!["Company: Honeywell Nigeria PLC\n\n".to_string()];
    let _shares =            vec!["Shares: 34,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 9,000,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1906\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 73.53%\n\n".to_string()];


    let mut file = std::fs::File::create("HONEYWELL FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Honeywell Nigeria PLC\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }
    

    let _company =           vec!["Company: Nigerian Breweries PLC\n\n".to_string()];
    let _shares =            vec!["Shares: 30,000,000\n\n".to_string()];
    let _liabilities =       vec!["Liabilities: 12,000,00\n\n".to_string()];
    let _year =              vec!["Founding year: 1946\n\n".to_string()];
    let _percentagelevrage = vec!["Percentage levrage: 60.00%\n\n".to_string()];


    let mut file = std::fs::File::create("NIGERIAN FILE.txt").expect("create failed");   
    
    file.write_all("This file contains all the information for Nigerian Breweries PLC\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._company.len() {
        file.write_all(_company[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._shares.len() {
        file.write_all(_shares[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._liabilities.len() {
        file.write_all(_liabilities[index].as_bytes()).expect("Failed to write into file");
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
    for index in 0.._year.len() {
        file.write_all(_year[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._percentagelevrage.len() {
        file.write_all(_percentagelevrage[index].as_bytes()).expect("Failed to write into file");
    }
    
    println!("\nAll done");
     
}

use std::io::Read;
use std::io::Write;
fn main() {
   
   use std::io;
      
      let mut input1 = String::new();
      let mut input2 = String::new();

    println!("Welcome to the Phantom Filing System");
    println!("\nPlease input your username with letters between a-z, minimum length - 3, maximum length - 8
              \nNOTE: Your username should be the first 4 letters of your company.");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let input1 = input1.trim();
    
    println!("\nInput your password with letters between a-z and numbers between 0-9
              \nmin. length of password: 3 ,max. length of password: 8");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let input2 = input2.trim();

    if input1 == "cadb" && input2 == "cadb12"{
    println!("Welcome to Cadbury Nigeria Plc");
    println!("YOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("CADBURY FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);}

    if input1 == "cham" && input2 == "cham12"{
    println!("Welcome to Champion Breweries Plc");
    println!("YOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("CHAMPION FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents); } 

    if input1 == "dang" && input2 == "dang12"{
    println!("Welcome to Dangote Sugar Refinery Plc");
    println!("YOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("DANGOTE FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);}

    if input1 == "flou" && input2 == "flou12"{
    println!("Welcome to Flour Mills Nigeria Plc");
    println!("YOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("FLOUR MILLS FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);}

    if input1 == "nest" && input2 == "nest12"{
    println!("Welcome to Nestle Nigeria Plc");
    println!("YOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("NESTLE FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);}

    if input1 == "unil" && input2 == "unil12"{
    println!("ACCESS GRANTED");
    println!("Welcome to Unilever Nigeria Plc");
    println!("YOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("UNILEVER FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);}

    if input1 == "hone" && input2 == "hone12"{
    println!("ACCESS GRANTED");
    println!("Welcome to Honeywell Nigeria Plc");
    println!("YOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("HONEYWELL FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);}

    if input1 == "nige" && input2 == "nige12"{
    println!("ACCESS GRANTED");
    println!("\nWelcome to Nigerian Breweries Plc"); 
    println!("\nYOU HAVE SUCCESSFULLY ACCESSED THIS FILE");
    let mut file = std::fs::File::open("NIGERIAN FILE.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);}
  }

     fn levrages() {

    let a = vec!["COMPANY NAME               ","LEVRAGE(%)"];
    let b = vec!["Champion Breweries PLC     ","68.00"];
    let c = vec!["Flour Mills Nigeria PLC    ","87.50"];
    let d = vec!["Unilever Nigeria PLC       ","70.27"];
    let e = vec!["Honeywell Nigeria PLC      ","73.53"];
    let f = vec!["Nigerian Breweries PLC     ","60.00"];

    let mut file = std::fs::File::create("COMPANIES WITH SHARES GREATER THAN 20,000,000.txt").expect("create failed");   
    file.write_all("This file contains all the percentage levrages of the companies with shares of more than 20,000,000\n\n".as_bytes()).expect("write failed");
    
    for index in 0..a.len() {
        file.write_all(a[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0..b.len() {
        file.write_all(b[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0..c.len() {
        file.write_all(c[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0..d.len() {
        file.write_all(d[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0..e.len() {
        file.write_all(e[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0..f.len() {
        file.write_all(f[index].as_bytes()).expect("Failed to write into file");
    }
    println!("\nAll done.");

}

    fn liabilities() {

     let a:f64 = 5500000.0;
     let b:f64 = 8000000.0;
     let c:f64 = 10000000.0;
     let d:f64 = 4000000.0;
     let e:f64 = 1500000.0;
     let f:f64 = 11000000.0;
     let g:f64 = 9000000.0;
     let h:f64 = 12000000.0;

     if a > 10000000.0{
           let _e1 = 0.50 * a;
           println!("{}",_e1);
     }

      if b > 10000000.0{
          let _e2 = 0.50 * b;
          println!("{}",_e2);
     }

      if c > 10000000.0{
           let _e3 = 0.50 * c;
           println!("{}",_e3);
      }

      if d > 10000000.0{
            let _e4 = 0.50 * d;
            println!("{}",_e4);
      }

      if e > 10000000.0{
             let _e5 = 0.50 * e;
             println!("{}",_e5); 
      }

      if f > 10000000.0{
            let _e6 = 0.50 * f;
            println!("{}",_e6);
      }

      if g > 10000000.0{
            let _e7 = 0.50 * g;
            println!("{}",_e7);
      }

      if h > 10000000.0{
             let _e8 = 0.50 * h;
             println!("{}",_e8);
      }
    }

    println!("Click 1 to access the list of percentage levrages of companies with a greater than 20,000,000 ");
    println!("Click 2 to access the list of companies with liabilities less than 10,000,000");

    let mut i12 = String::new();
    io::stdin().read_line(&mut i12).expect("Not a valid string");
    let input12:i32 = i12.trim().parse().expect("Not a valid number");

    if input12 == 1{
    levrage();
  }
    if input12 == 2{
    liabilities();
}


    
    

           




  
  