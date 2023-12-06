//Rust program to create a file
use std::io::Write;

 fn main() {

    let _lager = vec!["33 export\n".to_string(), "Desperados\n".to_string(), "Goldberg\n".to_string(), "Gulder\n".to_string(), "Heinken\n".to_string(), "Star\n\n".to_string()];
    let _stout = vec!["Legend\n".to_string(), "Turbo king\n".to_string(), "Williams\n\n".to_string()];
    let _non_alcoholic = vec!["Maltina\n".to_string(), "Amstel malta\n".to_string(), "Malta Gold\n".to_string(), "Fayrouz\n\n".to_string()];
    
    let mut file = std::fs::File::create("Drink Categories.txt").expect("create failed");

    file.write_all("Welcome to Nigerian Breweries plc,\nThese are our High-Quality Categories of Drinks \n\n".as_bytes()).expect("write failed");
    
    file.write_all("Under Lager drinks we have:\n\n".as_bytes()).expect("Failed to write into file");
    for index in 0.._lager.len() {
        file.write_all(_lager[index].as_bytes()).expect("Failed to write into file")
    }
     file.write_all("Under Stout drinks we have:\n\n".as_bytes()).expect("Failed to write into file");
    for index in 0.._stout.len() {
        file.write_all(_stout[index].as_bytes()).expect("Failed to write into file")
    }
    file.write_all("Under non-alcoholic drinks we have:\n\n".as_bytes()).expect("Failed to write into file"); 
    for index in 0.._non_alcoholic.len() {
        file.write_all(_non_alcoholic[index].as_bytes()).expect("Failed to write into file")

    }

    println!("\nAll done.");

    
    
    
         
 }
