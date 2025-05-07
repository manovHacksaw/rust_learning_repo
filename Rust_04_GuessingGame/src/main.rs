use std::io;
use rand::prelude::*;
// use rand::thread_rng; // ✅ Add this line


fn main() {
    let guess_list = ["grapes", "apples", "bananas", "pears", "kiwis"];
    let mut rng = rand::thread_rng(); 

    let index =rng.gen_range(0..guess_list.len());
    let guessed_fruit = guess_list[index];
    println!("Generated fruit: {}", guessed_fruit);

    let mut input = String::new();
    loop{
        match io::stdin().read_line(&mut input){
            Ok(_)=>{
                let fruit_selected = input.trim().to_lowercase();
                // println!("Fruit selected: {}", fruit_selected);
    
                if(!guess_list.contains(&fruit_selected.as_str())){
                    println!("Fruit not found in the list");
                    continue;
                }
    
               if guess_checker(&fruit_selected, &guessed_fruit){
                    println!("You guessed the fruit correctly");
                    break;
                }else{
                    println!("You guessed the fruit incorrectly, RETRY");
                    continue;
                }
            }
            Err(error)=>{
                println!("Error reading input, Error: {}", error);
            }
        }
    
    }
   
}


fn guess_checker(guessed_fruit: &str, generated_fruit: &str) -> bool{
    if guessed_fruit == generated_fruit{
        return true;
    }
    return false;
}