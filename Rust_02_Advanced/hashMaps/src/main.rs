use std::collections::HashMap;


fn main() {
    let mut users =    HashMap::new();

    // users.insert(String::from("Manobendra"), (1052));
    users.insert(String::from("Raj"), 1053);
    users.insert(String::from("Anuska"), 1054);
    
    let first_user_id = users.get("Manobendra");
    // println!("First User ID: {:?}", first_user_id);

    match first_user_id {
        Some(id) => println!("First User ID: {}", id),
        None => println!("No User ID found"),
    }
}
