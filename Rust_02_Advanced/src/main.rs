use std::collections::HashMap;

fn main() {
    // let mut vec: Vec<i32> = [1, 2, 3, 5].to_vec();
    // vec.push(6);
    // vec.push(9);
    // vec.push(4);
    // vec.push(8);
    // vec.push(16);
    // vec.push(15);

     // println!("{:?}", even_filter(&vec));
    // println!("{:?}", vec);


    let mut users = HashMap::new();
    users.insert(String::from("Deustch"), 32);
    users.insert(String::from("Deustch"), 31);

    match users.get("Deustch"){
        Some(value) => println!("We got {}", value),
        None => println!("We got nothing :(")

    } 
    




   
}


fn even_filter(arr: &Vec<i32>) -> Vec<i32>{
    let mut res: Vec<i32> = Vec::new();
    for val in arr{
        if val % 2 == 0{
            res.push(*val);
        }
    }

    return res;
    

}