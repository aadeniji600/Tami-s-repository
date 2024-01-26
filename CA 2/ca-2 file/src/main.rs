  //Rust program 
 
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





  


  
