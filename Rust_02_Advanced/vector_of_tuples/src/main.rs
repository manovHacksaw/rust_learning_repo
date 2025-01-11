use std::collections::HashMap;

fn main() {
    // Define a tuple
    let tuple: (i32, String) = (21, String::from("Manobendra"));
    println!("Tuple: {:?}", tuple);

    // Define a vector of tuples
    let vector_of_tuples: Vec<(i32, String)> = vec![
        (21, String::from("Manobendra")),
        (34, String::from("Pawan")),
        (12, String::from("Ang Thilen")),
    ];

    // Group by values using the function
    let grouped_map = group_by_values(vector_of_tuples);

    // Print the resulting HashMap
    for (key, value) in grouped_map {
        println!("Key: {}, Value: {}", key, value);
    }
}

// Function to group values into a HashMap
fn group_by_values(input: Vec<(i32, String)>) -> HashMap<i32, String> {
    let mut res: HashMap<i32, String> = HashMap::new();
    for (key, value) in input {
        res.insert(key, value); // Insert key-value pairs into the HashMap
    }
    res
}
