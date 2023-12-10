//Rust program to display details of students and save it in a file
 use std::io::Write;

  fn main() {
      
    let _heading = vec!["STUDENT NAME   ".to_string(),"MATRIC. NUMBER  ".to_string(), "DEPARTMENT  ".to_string(), " LEVEL\n\n".to_string()];
    let _student1 = vec!["Oluchi Mordi  ".to_string(), "ACC10211111    ".to_string(), "Accounting  ".to_string(), " 300\n\n".to_string()];
    let _student2 = vec!["Adams Aliyu   ".to_string(), "ECO10110101    ".to_string(), "Economics   ".to_string(), " 100\n\n".to_string()];
    let _student3 = vec!["Shania Bolade  ".to_string(), "CSC10328828   ".to_string(), "Computer    ".to_string(), " 200\n\n".to_string()];
    let _student4 = vec!["Adekunle Gold  ".to_string(), "EEE11020202   ".to_string(), "Electrical  ".to_string(), " 200\n\n".to_string()];
    let _student5 = vec!["Bianca Edemoh  ".to_string(), "MEE10202001   ".to_string(), "Mechanical  ".to_string(), " 100\n\n".to_string()];

    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("create failed");   
    
    file.write_all("Welcome to PAU-SMIS\n\n".as_bytes()).expect("write failed");
    
    for index in 0.._heading.len() {
        file.write_all(_heading[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._student1.len() {
        file.write_all(_student1[index].as_bytes()).expect("Failed to write into file");
    }
    
    for index in 0.._student2.len() {
        file.write_all(_student2[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._student3.len() {
        file.write_all(_student3[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._student4.len() {
        file.write_all(_student4[index].as_bytes()).expect("Failed to write into file");
    }

    for index in 0.._student5.len() {
        file.write_all(_student5[index].as_bytes()).expect("Failed to write into file");
    }
    
    println!("\nAll done");
     
        
  }

